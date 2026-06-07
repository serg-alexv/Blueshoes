use crate::profiles::schema::{ProfileSchema, ProfileIntent};
use serde::{Serialize, Deserialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action")]
pub enum PlanStep {
    AddRoute { target: String, via: String },
    FlushRouteCache,
    AddNftRule { table: String, chain: String, rule: String },
    SetMtu { interface: String, mtu: u32 },
}

pub struct Planner;

impl Planner {
    pub fn plan(profile: &ProfileSchema) -> Result<Vec<PlanStep>, String> {
        let mut steps = Vec::new();

        match profile.intent {
            ProfileIntent::DnsPrivacy => {
                if let Some(dns) = &profile.dns {
                    for ip_str in &dns.upstream_ips {
                        // Strict input validation: Ensure it is a valid IP address
                        let ip = ip_str.parse::<IpAddr>()
                            .map_err(|_| format!("Invalid IP address in DnsPrivacy profile: {}", ip_str))?;
                        
                        steps.push(PlanStep::AddRoute { 
                            target: ip.to_string(), 
                            via: "10.0.0.1".to_string() 
                        });
                    }
                    // Add safe NFT rule
                    steps.push(PlanStep::AddNftRule { 
                        table: "inet".to_string(), 
                        chain: "forward".to_string(), 
                        rule: "tcp dport 853 accept".to_string() 
                    });
                } else {
                    return Err("DnsPrivacy intent requires 'dns' configuration".to_string());
                }
            }
            ProfileIntent::EchPreserve => {
                steps.push(PlanStep::AddNftRule { 
                    table: "inet".to_string(), 
                    chain: "forward".to_string(), 
                    rule: "tcp dport 443 accept".to_string() 
                });
            }
            ProfileIntent::UserTunnel => {
                steps.push(PlanStep::AddRoute { 
                    target: "0.0.0.0/0".to_string(), 
                    via: "wg0".to_string() 
                });
            }
            ProfileIntent::SafeMtu => {
                steps.push(PlanStep::SetMtu { 
                    interface: "pppoe-wan".to_string(), 
                    mtu: 1492 
                });
            }
            ProfileIntent::RecoverySafeMode => {
                steps.push(PlanStep::FlushRouteCache);
            }
        }

        Ok(steps)
    }

    pub fn dry_run(profile: &ProfileSchema) -> Result<Vec<String>, String> {
        let steps = Self::plan(profile)?;
        Ok(steps.into_iter().map(|s| serde_json::to_string(&s).unwrap()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::schema::DnsAction;

    #[test]
    fn test_valid_dns_privacy_plan() {
        let profile = ProfileSchema {
            name: "test".to_string(),
            description: "test".to_string(),
            intent: ProfileIntent::DnsPrivacy,
            dns: Some(DnsAction { 
                upstream_ips: vec!["1.1.1.1".to_string(), "9.9.9.9".to_string()],
                dot_hostname: None, 
            }),
            routes: None,
        };
        let plan = Planner::plan(&profile).unwrap();
        assert_eq!(plan.len(), 3); // 2 routes, 1 nft rule
    }

    #[test]
    fn test_invalid_ip_dns_privacy() {
        let profile = ProfileSchema {
            name: "test".to_string(),
            description: "test".to_string(),
            intent: ProfileIntent::DnsPrivacy,
            dns: Some(DnsAction { 
                upstream_ips: vec!["1.1.1.1".to_string(), "invalid_ip; rm -rf /".to_string()],
                dot_hostname: None,
            }),
            routes: None,
        };
        let result = Planner::plan(&profile);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid IP address"));
    }
}
