use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ExtendedACLs {
    pub r#type: String,
    pub access_lists: HashMap<String, ExtendedACL>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ExtendedACL {
    pub sequence_numbers: HashMap<i32, AccessListEntry>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AccessListEntry {
    pub action: String,
}

pub fn permit_from_ips(ips: &[crate::nautobot::IpAddressType]) -> ExtendedACLs {
    // TODO: add params for name of acl and type
    let mut access_lists = HashMap::new();
    let mut seqn = HashMap::new();
    for (i, ip) in ips.iter().enumerate() {
        let action = format!("permit ip any {}", ip.address);
        let ale = AccessListEntry { action };
        seqn.insert((i as i32 + 1) * 10, ale);
    }
    let sacl = ExtendedACL {
        sequence_numbers: seqn,
    };
    access_lists.insert("critical".to_owned(), sacl);
    let r#type = "l3leaf".to_owned();
    ExtendedACLs {
        r#type,
        access_lists,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_permit_from_ips() {
        let ips = vec![crate::nautobot::IpAddressType {
            address: "10.1.1.0/24".to_owned(),
        }];
        let ale = AccessListEntry {
            action: "permit ip any 10.1.1.0/24".to_owned(),
        };
        let sequence_numbers: HashMap<i32, AccessListEntry> = [(10, ale)].iter().cloned().collect();
        let eacl = ExtendedACL { sequence_numbers };
        let access_lists: HashMap<String, ExtendedACL> =
            [("critical".to_owned(), eacl)].iter().cloned().collect();
        let acl = ExtendedACLs {
            r#type: "l3leaf".to_owned(),
            access_lists,
        };
        let result = permit_from_ips(&ips);
        assert_eq!(result, acl);
    }
}
