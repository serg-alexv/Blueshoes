use crate::profiles::schema::{ProfileSchema, ProfileIntent};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum PlanStep {
    AddRoute { target: String, via: String },
    FlushRouteCache,
    AddNftRule { table: String, chain: String, rule: String },
    SetMtu { interface: String, mtu: u32 },
}

pub struct Planner;

impl Planner {
    pub fn plan(profile: &ProfileSchema) -> Vec<PlanStep> {
        let mut steps = Vec::new();

        match profile.intent {
            ProfileIntent::DnsPrivacy => {
                if let Some(dns) = &profile.dns {
                    for ip in &dns.upstream_ips {
                        steps.push(PlanStep::AddRoute { target: ip.clone(), via: "10.0.0.1".to_string() });
                    }
                    steps.push(PlanStep::AddNftRule { table: "inet".to_string(), chain: "forward".to_string(), rule: "tcp dport 853 accept".to_string() });
                }
            }
            ProfileIntent::EchPreserve => {
                steps.push(PlanStep::AddNftRule { table: "inet".to_string(), chain: "forward".to_string(), rule: "tcp dport 443 accept".to_string() });
            }
            ProfileIntent::UserTunnel => {
                steps.push(PlanStep::AddRoute { target: "0.0.0.0/0".to_string(), via: "wg0".to_string() });
            }
            ProfileIntent::SafeMtu => {
                steps.push(PlanStep::SetMtu { interface: "pppoe-wan".to_string(), mtu: 1492 });
            }
            ProfileIntent::RecoverySafeMode => {
                steps.push(PlanStep::FlushRouteCache);
            }
        }

        steps
    }

    pub fn dry_run(profile: &ProfileSchema) -> Vec<String> {
        let steps = Self::plan(profile);
        steps.into_iter().map(|s| serde_json::to_string(&s).unwrap()).collect()
    }
}
