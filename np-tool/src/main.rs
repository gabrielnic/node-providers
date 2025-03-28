pub mod addresses;
pub mod helper;
pub mod transactions;

use addresses::{exchanges::EXCHANGES, node_providers::NODE_PROVIDERS, snses::SNSES, spammers::SPAMMERS};
use candid::Principal;
use ic_agent::Agent;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error as ThisError;
use transactions::fetch_account_transactions;

const IC_URL: &str = "https://ic0.app";
///
/// Error
///

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Principal error: {0}")]
    Principal(#[from] ic_agent::export::PrincipalError),
}

///
/// AccountData
///

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
    name: String,
    principal: Option<Principal>,
    account: Option<String>,
    ty: Type,
}

impl AccountData {
    pub fn new(name: &str, address: &str, ty: Type) -> Self {
        let (principal, account) = if address.contains("-") {
            (Some(Principal::from_text(address).unwrap()), None)
        } else {
            (None, Some(address.to_string()))
        };

        Self { name: name.to_string(), principal, account, ty }
    }
}

///
/// AccountType
///

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Exchange,
    Foundation,
    Individual,
    NodeProvider,
    Spammer,
    Sns,
    Suspect,
}

//
// main
//

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = Agent::builder().with_url(IC_URL).build()?;

    // Initialize the agent (fetch root key in development)
    agent.fetch_root_key().await?;

    let mut results = Vec::new();
    for entry in get_entries() {
        match fetch_account_transactions(entry, &agent).await {
            Ok(account_tx) => results.push(account_tx),
            Err(e) => eprintln!("Error fetching account transactions: {}", e),
        }
    }

    let json_string = serde_json::to_string_pretty(&results)?;
    std::fs::write("./../frontend/public/account_transactions.json", json_string)?;
    println!("Saved combined account transactions to account_transactions.json");

    Ok(())
}

const FOUNDATION: &[&str] = &[
    "1a481b7aa86fa89029117f0487479f52fe3ee27ad63d48bac78fa60b3ebf9237",
    "12b7c1c6269b2021758ed5da65448a4ac3ac9fa0cf015caa4bb4c3e3dc7ca1c1",
    "1f5ef4de27f2880dac6409b431ba866170be8b04e87afe95c841ca163d9c3689",
    "21b3cb5fccbbb1b4d92c03ace6f16f836e3425cf61ac3b2a9823499a60d5c7b5",
    "36039b216d234b39bfc993df251deba6d7481d974f20a7aa4ea8a2aff8c7606e",
    "406ae771636e9e6501166f71edb0b61b80a325640048b11b23b3eaf43a5048ed",
    "5495612bb400e0dbd51ca4ae28835b3c47f6968127fe54d4aadc8704cc363057",
    "54f4a6d3bc831c5217e4e313bca7f2549f6b51b9dc25d77ae045bbb59c8eafaf",
    "57c9e0f1111d0aba921adf0056a16771e9a4fd84a6daee21267511b2b3410731",
    "581ebcfa72dbed72cb0d18240d30879ac915df69dba3d1a8cfbb5508bb973df1",
    "600bc2e6fa9dcf3543acc22bd8181ca7e11ef3f0a9ce662480fe61524c9bb8c1",
    "61529f442b6fc6a2db07f4dc446b255e6702aa95ed8bb2fa5c38cb04358eea65",
    "65675acf9e0752bbc58893b284e2c7558db61bfdbefe63c055bb69305f0da17e",
    "6a6fe5e9936747faadb472f3d3790830c372eb4ab7039d207059bf34eacf4de9",
    "78a6c47cc1e153e1b63eafcd471d5e6de1365592e73d0c41931461fd97271d03",
    "860c10fd2e96106edccb48e657b265517fda354a4588a52a13384dd58ffcaab2",
    "91ee71b84b7603de21bcf6cc1857a7cb91d12062d91f95013d300e0eb35e6d52",
    "ad2d4c5c3b70fa1289c7b45ef252d608f8d70f0d9b8198d6a6d0c4f5bdfc9c65",
    "b0c25df9be777bb84a2b8ddec02bbf42bfef588c44bc7fd483a515725cd68830",
    "b30d4f02181bd81dbe5ef9f22b33a7d7a5fe61884964fc697721ca259cfd3c41",
    "b93164c6ae75984345a3d47fa9877749acb19fee032ec6aac67cb3ee6100f302",
    "b9cdb3a04e388736de3eea9426f4f83e79c90a6602b0805746ff7991e546df9f",
    "d5336412e2107f4b0502234128dbc11ccf53221ae67bae5109eb4be11bb9babb",
    "d539266d9e7784304ceb7b72a729794004646e4a39a56b51c3e911a698bbdd8d",
    "f7d23ad118bab9eae59055a98addd2a1738cc281fa1dc7ca4568e8e661e21283",
];

const INDIVIDUALS: &[(&str, &str)] = &[
    ("Austin Fatheree", "jrnhz-6ekxv-2fffs-wfcgt-l3pe7-456id-heznf-xyf64-nykjq-4jyso-zae"),
    ("Johannes Kriel", "2rjjb-gy24i-ghulj-zfsn6-cf6ju-6rrkc-osdlt-uxuhc-ibhmb-wvh2v-yae"),
    ("Gavin H 1", "7cfaeaa0e14ce862636f052ff307511032e030a3028ead7614f7fc0905c1de41"),
    ("Utkarsh Goyal (GoBazzinga)", "736130c585e271287afc27a381acfcf5c2757203763caad6bf70be0e98a6a0e6"),
    ("IcDevs Tax Deductible", "c8e78c28beebd305370cfc798bbe96132fcade8ac88ec742084a4e6e248913cf"),
    ("IcDevs Anonymous", "93a3506e08e88e1f65f85990451038f7a232b6f860ee706b42cc9edec96eecee"),
    ("IcDevs NFTs", "p75el-ys2la-2xa6n-unek2-gtnwo-7zklx-25vdp-uepyz-qhdg7-pt2fi-bqe"),
    ("Toniq Royalty", "c7e461041c0c5800a56b64bb7cefc247abc0bbbb99bd46ff71c64e92d9f5c2f9"),
];

const SUSPECTS: &[(&str, &str)] = &[
    ("Genesis Whale (2000) 1", "73a3e56c7177c29c731618b1c60cfeb271c00d70ae40aba9202cdec84e977d39"),
    ("Genesis Whale (2000) 2", "843187c470d88e1b0958840c768d7592b140e4c93a0359388cc0e69c6a653833"),
    ("Genesis Whale (2000) 3", "5a15ff1832772182e35bc73e53cd372286ca5185beed546989485349a211b798"),
    ("Genesis Whale (2000) 4", "8b8fff2a81588e1c095af6cb9c69acc031e8bd5e2483887aceba5872e19f2424"),
    ("Genesis Whale (2000) 5", "f7641b665a8275f61c91cb743754ff2e6f575c68477fc351d101eb74eab7f042"),
    ("Genesis Whale (2000) 6", "573501760b5e1654dbf24852f0045426586d96f00ffd13a212f2e9cc820c0630"),
    ("Genesis Whale (2000) 7", "eefb4d05d68c147f596d9718c7336b08b0bbbd4f2d5be692b7072904b4c1fd1a"),
    ("Genesis Whale (2000) 8", "25e4a7d6d45cf52c9ec02cf1fdf2f1118e3843a47f3f94817031c45170aa24b8"),
    ("Genesis Whale (2000) 9", "1055f803a4c8e19fa863c1933281b778732ffaa50b72e0e7bc8d2db25ed57ee4"),
    ("Genesis Whale (2000) 10", "8aeb77c9e83bd3063ee576ad97b37b893bad401d43b3a66822ae3b700a5d2085"),
    ("Genesis Whale 1", "5257f7dc8da3ab4850f4d299b5ca34f29b89f149a834099d0bd9fecab27a537d"),
    ("Genesis Whale 2", "125013e95bd5e008bd6d26f86f5ddda2b16c382372b3067672505c1f11418817"),
    ("Genesis Whale (10501) 1", "8ef1325bc363e8ee2d73079cf9bcd56bc0991f72715f8b229b248ba3133a0782"),
    ("Genesis Whale (10501) 2", "06ccfd22a47cf0f0b149806bf551e5646f896f07e228d44724ea88563191d8d5"),
    ("Genesis Whale (10501) 3", "89a1b4f7ebb8dc35b6b830b9fd48a6163fa5e04eba5747d760e9ea596ee24d71"),
    ("Genesis Whale (10501) 4", "f42ef05c1c99e40dc01a08b5a27a6277c2bce74ad498f322c6b6cabd7ec54627"),
    ("Genesis Whale (10501) 5", "3f8de2ecb6c011ec265aec0ce9a23abf0278c07d0471d24e956f704fe0e63118"),
    ("Genesis Whale (10501) 1", "a4d4c3b7847ffd3188d659b85fc29836dc98bb183f9482225f6254634c4fb770"),
    ("Genesis Mixer 1", "05ad474665f1eec0714c1a4ec941c3a395c703e14bb43100bd946d80b87828af"),
    ("BIL Hacker", "3axar-twhdo-biizl-yegt2-fatxq-go2ay-ib5ki-y6cmq-ziiav-vcn5x-mae"),
    ("BIL Hacker ckBTC Account", "az453-x2sxf-wewfl-pszbd-4u4rh-yq7nk-hxkrp-6yvo3-mnlce-zjvsg-qae"),
    ("Weird Bot 1", "ddc050bf2a59f2d905f0c7af45854cd4cc4e406c643c322e5fa65e83a36d97da"),
    ("Weird Bot 2", "4ec84f148280c743948b2f54911bbcdcbc6996169f20b52eafd03544d03453fa"),
    ("ufwij", "ufwij-jggzv-owfkb-cs26m-p7j3y-awpqg-3oa33-x4ciu-vadlo-2jb7f-gae"),
    ("NF 1 (1.3m ICP)", "lsyd6-e7avj-lnf7q-fqga7-nb3x4-gum2h-fajff-4urd5-gve2l-tppm2-7ae"),
    ("NF 2 (1.1m ICP)", "yjjc4-kc4ge-io5mm-m5kye-pcm2v-qwgci-yn7zh-tyj6w-ur33e-ncsmx-xae"),
    ("NF 3 (796k ICP)", "bqjsc-ygbpe-gtqrs-nq3mf-d4iot-n2m7r-cfld2-iynvs-ls5qf-ffu2w-vqe"),
    ("NF 4 (771k ICP)", "hrpgd-p2dys-gd5tb-krk4d-nswtt-un5h3-x6btw-j4sdm-wvscw-o2yej-iqe"),
    ("NF 5 (223k ICP)", "4vnki-cqaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aaaaa-aae"),
    ("NF 6 (44k ICP)", "rdwk2-noc2n-qaxh6-3alc4-uvhgt-dupge-kkoq3-v3brf-6afky-mui7j-lqe"),
    ("NF 10 (1845 ICP)", "afxjy-xzged-ttm2u-5rjp7-exday-s6uly-ea4pc-xkiok-tjzva-23isp-vae"),
    ("NF 11 (986 ICP)", "byfqe-a6vvd-vxehg-k5hi3-ij3v3-7n6qv-smmxm-v3vg7-mye6g-thgrs-kae"),
    ("NF 12 (660 ICP)", "c4dgi-zb67y-vgmq3-gpm55-szzjo-mc3kt-jjov3-yytoy-ltq6t-ptyyv-lqe"),
    ("NF 13 (572 ICP)", "amatj-baend-pdd4b-tantp-b3heu-uvusn-abmj5-hkhf2-xlvfm-jy6xp-uae"),
    ("NF 14 (557 ICP)", "etynm-5engo-23sxo-jlss2-7jnkl-zxqv2-3s3s7-w7kpt-uaqnb-ckg6m-rae"),
    ("NF 15 (411 ICP)", "bgmtq-s5ra3-l4ftn-zmi5f-wg2o4-zolb4-pyyez-hyttd-7rvuw-r3gyl-4ae"),
    ("NF 16 (396 ICP)", "oggca-p5idg-tq22l-meqsr-kupbo-m3lpf-h6wi7-zplva-coxgr-tm3vt-2qe"),
    ("Approver 1", "6202e0cfffbbb22acd373aba740d2c10d84a1c6b044b97fe4f649c9c7a2426b6"),
    ("Possible Mixer", "2117d6be92d5c43adae0443cca9d30409d52acdf17eead1e63916e46ca891c37"),
];

// get_entries
fn get_entries() -> Vec<AccountData> {
    let mut entries = Vec::new();

    // named
    entries.extend(EXCHANGES.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Exchange)));
    entries.extend(INDIVIDUALS.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Individual)));
    entries.extend(NODE_PROVIDERS.iter().map(|(name, addr)| AccountData::new(name, addr, Type::NodeProvider)));
    entries.extend(SNSES.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Sns)));
    entries.extend(SUSPECTS.iter().map(|(name, addr)| AccountData::new(name, addr, Type::Suspect)));

    // no name
    entries.extend(FOUNDATION.iter().map(|addr| AccountData::new(&addr[..5], addr, Type::Foundation)));
    entries.extend(SPAMMERS.iter().map(|addr| AccountData::new(&addr[..5], addr, Type::Spammer)));

    // check for dupes
    let mut seen_account_ids = HashSet::new();
    let mut seen_principals = HashSet::new();
    print!("Validating {} addresses...", entries.len());
    for entry in &entries {
        if let Some(acc) = &entry.account {
            if !seen_account_ids.insert(acc) {
                panic!("duplicate account found: {acc}");
            }
        }

        if let Some(pid) = &entry.principal {
            if !seen_principals.insert(pid) {
                panic!("duplicate principal found: {pid}");
            }
        }
    }
    println!(" ok");

    entries
}
