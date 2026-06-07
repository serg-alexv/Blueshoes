use crate::profiles::schema::{ProfileSchema, ProfileIntent};
use serde::{Serialize, Deserialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftFamily { Inet }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftTable { Filter }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftChain { Forward }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportProtocol { Tcp, Udp }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftAction { Accept, Drop, Reject }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action")]
pub enum PlanStep {
    AddRoute { target: String, via: String },
    FlushRouteCache,
    AddNftRule { 
        family: NftFamily,
        table: NftTable, 
        chain: NftChain, 
        protocol: TransportProtocol, 
        dport: u16, 
        rule_action: NftAction 
    },
    SetMtu { interface: String, mtu: u32 },
}

const ALLOWED_INTERFACES: &[&str] = &["pppoe-wan", "wan", "wg0", "br-lan"];

fn is_valid_interface_or_ip(val: &str) -> bool {
    ALLOWED_INTERFACES.contains(&val) || val.parse::<IpAddr>().is_ok()
}

pub struct Planner;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunEvidence {
    pub execution_mode: String,
    pub mutation_performed: bool,
    pub profile_name: String,
    pub plan_steps: Vec<PlanStep>,
    pub plan_sha256: String,
    pub refusal_reason: String,
}

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
                        
                        let via = "10.0.0.1".to_string();
                        if !is_valid_interface_or_ip(&via) {
                            return Err(format!("Invalid via interface/IP: {}", via));
                        }
                        steps.push(PlanStep::AddRoute { 
                            target: ip.to_string(), 
                            via 
                        });
                    }
                    // Add safe NFT rule
                    steps.push(PlanStep::AddNftRule { 
                        family: NftFamily::Inet,
                        table: NftTable::Filter, 
                        chain: NftChain::Forward, 
                        protocol: TransportProtocol::Tcp,
                        dport: 853,
                        rule_action: NftAction::Accept,
                    });
                } else {
                    return Err("DnsPrivacy intent requires 'dns' configuration".to_string());
                }
            }
            ProfileIntent::EchPreserve => {
                steps.push(PlanStep::AddNftRule { 
                    family: NftFamily::Inet,
                    table: NftTable::Filter, 
                    chain: NftChain::Forward, 
                    protocol: TransportProtocol::Tcp,
                    dport: 443,
                    rule_action: NftAction::Accept,
                });
            }
            ProfileIntent::UserTunnel => {
                let via = "wg0".to_string();
                if !is_valid_interface_or_ip(&via) {
                    return Err(format!("Invalid via interface/IP: {}", via));
                }
                steps.push(PlanStep::AddRoute { 
                    target: "0.0.0.0/0".to_string(), 
                    via 
                });
            }
            ProfileIntent::SafeMtu => {
                let interface = "pppoe-wan".to_string();
                if !is_valid_interface_or_ip(&interface) {
                    return Err(format!("Invalid interface: {}", interface));
                }
                steps.push(PlanStep::SetMtu { 
                    interface, 
                    mtu: 1492 
                });
            }
            ProfileIntent::RecoverySafeMode => {
                steps.push(PlanStep::FlushRouteCache);
            }
        }

        Ok(steps)
    }

    pub fn dry_run(profile: &ProfileSchema, refusal_reason: &str) -> Result<DryRunEvidence, String> {
        use sha2::{Sha256, Digest};
        let steps = Self::plan(profile)?;
        
        let steps_json = serde_json::to_string(&steps).map_err(|e| e.to_string())?;
        let mut hasher = Sha256::new();
        hasher.update(steps_json.as_bytes());
        let hash_result = hasher.finalize();
        let plan_sha256 = hash_result.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        Ok(DryRunEvidence {
            execution_mode: "dry_run".to_string(),
            mutation_performed: false,
            profile_name: profile.name.clone(),
            plan_steps: steps,
            plan_sha256,
            refusal_reason: refusal_reason.to_string(),
        })
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
