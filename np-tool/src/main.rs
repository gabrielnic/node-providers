pub mod helper;
pub mod transactions;

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
    Sns,
    Spammer,
    Suspect,
}

// main
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    let agent = Agent::builder().with_url(IC_URL).build()?;

    // Initialize the agent (fetch root key in development)
    agent.fetch_root_key().await?;
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

const EXCHANGES: &[(&str, &str)] = &[
    ("Bitget", "bad030b417484232fd2019cb89096feea3fdd3d9eb39e1d07bcb9a13c7673464"),
    ("Binance 1", "609d3e1e45103a82adc97d4f88c51f78dedb25701e8e51e8c4fec53448aadc29"),
    ("Binance 2", "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5"),
    ("Binance 3", "d3e13d4777e22367532053190b6c6ccf57444a61337e996242b1abfb52cf92c8"),
    ("Bybit", "acd76fff0536f863d9dd4b326a1435466f82305758b4b1b4f62ff9fa81c14073"),
    ("Coinbase 1", "449ce7ad1298e2ed2781ed379aba25efc2748d14c60ede190ad7621724b9e8b2"),
    ("Coinbase 2", "4dfa940def17f1427ae47378c440f10185867677109a02bc8374fc25b9dee8af"),
    ("Coinbase 3", "dd15f3040edab88d2e277f9d2fa5cc11616ebf1442279092e37924ab7cce8a74"),
    ("Coinbase (Inactive 2021) 1", "a6ed987d89796f921c8a49d275ec7c9aa04e75a8fc8cd2dbaa5da799f0215ab0"),
    ("Coinbase (Inactive 2021) 2", "660b1680dafeedaa68c1f1f4cf8af42ed1dfb8564646efe935a2b9a48528b605"),
    ("Coinbase (Inactive 2021) 3", "4878d23a09b554157b31323004e1cc053567671426ca4eec7b7e835db607b965"),
    ("Gate.io", "8fe706db7b08f957a15199e07761039a7718937aabcc0fe48bc380a4daf9afb0"),
    ("HTX", "935b1a3adc28fd68cacc95afcdec62e985244ce0cfbbb12cdc7d0b8d198b416d"),
    ("Kraken", "040834c30cdf5d7a13aae8b57d94ae2d07eefe2bc3edd8cf88298730857ac2eb"),
    ("KuCoin 1", "efa01544f509c56dd85449edf2381244a48fad1ede5183836229c00ab00d52df"),
    ("KuCoin 2", "00c3df112e62ad353b7cc7bf8ad8ce2fec8f5e633f1733834bf71e40b250c685"),
    ("MEXC", "9e62737aab36f0baffc1faac9edd92a99279723eb3feb2e916fa99bb7fe54b59"),
    ("OKX 1", "e7a879ea563d273c46dd28c1584eaa132fad6f3e316615b3eb657d067f3519b5"),
    ("OKX 2", "d2c6135510eaf107bdc2128ef5962c7db2ae840efdf95b9395cdaf4983942978"),
];

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
    ("ufwij", "ufwij-jggzv-owfkb-cs26m-p7j3y-awpqg-3oa33-x4ciu-vadlo-2jb7f-gae"),
];

const NODE_PROVIDERS: &[(&str, &str)] = &[
    ("0X52", "2wxzd-qrbrs-ailta-kdtyb-ucg35-xcxd4-txevb-ot7hx-wiyus-szcca-nqe"),
    ("100 Count Holdings, LLC", "2dgp4-h57n4-a4kgx-n4uun-huo3a-wbdlc-m57wd-jtkuh-g5vcc-fcbby-6qe"),
    ("43rd Big Idea Films", "sqhxa-h6ili-qkwup-ohzwn-yofnm-vvnp5-kxdhg-saabw-rvua3-xp325-zqe"),
    ("87m Neuron, LLC", "eipr5-izbom-neyqh-s3ec2-52eww-cyfpg-qfomg-3dpwj-4pffh-34xcu-7qe"),
    ("ACCUSET SOLUTIONS", "cp5ib-twnmx-h4dvd-isef2-tu44u-kb2ka-fise5-m4hta-hnxoq-k45mm-hqe"),
    ("Aitubi AG", "znw2p-4cx6u-ocqls-277iu-2lkir-xjy7g-4s3sj-sjy6j-mtlay-rnnra-yqe"),
    ("Aksinia Stavskaya", "wlxga-ebupj-sj2nf-g3sii-75i6b-oh64s-qmq7u-gmros-vi2if-3ktdv-cqe"),
    ("Allusion", "rbn2y-6vfsb-gv35j-4cyvy-pzbdu-e5aum-jzjg6-5b4n5-vuguf-ycubq-zae"),
    ("Anonstake", "kos24-5xact-6aror-uofg2-tnvt6-dq3bk-c2c5z-jtptt-jbqvc-lmegy-qae"),
    ("ANYPOINT PTY LTD", "fwnmn-zn7yt-5jaia-fkxlr-dzwyu-keguq-npfxq-mc72w-exeae-n5thj-oae"),
    ("Arceau NP LLC", "ss6oe-fm7b2-b5r57-y3x74-omrz5-d5pgy-5iwtw-4aew5-aqj3l-6ydra-wqe"),
    ("Artem Horodyskyi", "diyay-s4rfq-xnx23-zczwi-nptra-5254n-e4zn6-p7tqe-vqhzr-sd4gd-bqe"),
    ("Aspire Properties", "2byzn-q2crt-hgczo-eruff-6p7af-pemor-n2z4z-6d2sd-wvdqa-yqvxb-mqe"),
    ("AVRVM AG", "33aps-ovxje-mwpux-cy2hh-f2qwp-5tzxs-2edbb-gblfn-ev5pv-cfnvj-pqe"),
    ("Bianca-Martina Rohner", "eatbv-nlydd-n655c-g7j7p-gnmpz-pszdg-6e6et-veobv-ftz2y-4m752-vqe"),
    ("Bigger Capital", "7a4u2-gevsy-5c5fs-hsgri-n2kdz-dxxwf-btcfp-jykro-l4y7c-7xky2-aqe"),
    ("Bitmoon ", "mjnyf-lzqq6-s7fzb-62rqm-xzvge-5oa26-humwp-dvwxp-jxxkf-hoel7-fqe"),
    ("Blockchain Development Labs", "7at4h-nhtvt-a4s55-jigss-wr2ha-ysxkn-e6w7x-7ggnm-qd3d5-ry66r-cae"),
    ("BlockFinance", "c5svp-7pkmf-agz5x-536k7-r7rcw-4wn3a-eo7pt-ry7su-j42uq-bvnzf-iqe"),
    ("BlockTech Ventures, LLC", "ks7ow-zvs7i-ratdk-azq34-zio2b-gbekj-qjicg-pfhp3-ovhgu-k5qql-dae"),
    ("BLP22, LLC", "sma3p-ivkif-hz7nu-ngmvq-ibnjg-nubke-zf6gh-wbnfc-2dlng-l3die-zqe"),
    ("Blue Ant LLC", "rpfvr-s3kuw-xdqrr-pvuuj-hc7hl-olytw-yxlie-fmr74-sr572-6gdqx-iqe"),
    ("Bohatyrov Volodymyr", "dhywe-eouw6-hstpj-ahsnw-xnjxq-cmqks-47mrg-nnncb-3sr5d-rac6m-nae"),
    ("Buldakova Rehina", "qipsq-44ztq-4oxob-dulxs-35tho-zjf5o-onu2b-sjuhk-4jd7x-yfdhz-qae"),
    ("Carbon Twelve", "qsdw4-ao5ye-6rtq4-y3zhm-icjbj-lutd2-sbejz-4ajqz-pcflr-xrhsg-jae"),
    ("Conic Ventures", "i3cfo-s2tgu-qe5ym-wk7e6-y7ura-pptgu-kevuf-2feh7-z4enq-5hz4s-mqe"),
    ("Coplus Limited", "jz47c-irtey-dr2nb-wienh-emhaz-jo6ua-gsbho-t2z5j-l7kbf-5i7p5-5ae"),
    ("Decentralized Entities Foundation", "w4buy-lgwzr-pccs7-huzhh-qqnws-rns75-iaoox-jolrm-xs2ra-vdu3o-2qe"),
    ("DeNoDe", "acqus-l4yyc-h44lw-grfxw-h7jqf-mtvt3-huwmj-4s372-sc5db-5nsfr-2ae"),
    ("DFINITY Stiftung", "bvcsg-3od6r-jnydw-eysln-aql7w-td5zn-ay5m6-sibd2-jzojt-anwag-mqe"),
    ("DFINITY USA Research LLC", "lq5ra-f4ibl-t7wpy-hennc-m4eb7-tnfxe-eorgd-onpsl-wervo-7chjj-6qe"),
    ("DFINITY USA Research LLC", "r2qgy-abcek-yc2ot-3powq-gayr6-4dibc-jvvfl-ri376-gmfmw-u2v7w-4qe"),
    ("Eastman Ventures (Pty) Ltd", "veamq-6zmtx-dtdky-ctoun-gokvu-cr6zm-ffsky-dz35w-e2euw-zvv7e-vae"),
    ("Exaion", "xo7ih-nswlt-hbq3n-v5ixi-etu7j-sasg6-fjf4p-zx6or-cc7c3-pnh7t-2ae"),
    ("Extragone SA", "7ryes-jnj73-bsyu4-lo6h7-lbxk5-x4ien-lylws-5qwzl-hxd5f-xjh3w-mqe"),
    ("Ferndale International", "34cav-6s7rb-uwa3c-awdly-5md4r-lwueh-atzbn-unqpe-c5ope-f3nqj-wae"),
    ("Fractal Labs AG", "wdjjk-blh44-lxm74-ojj43-rvgf4-j5rie-nm6xs-xvnuv-j3ptn-25t4v-6ae"),
    ("Geeta Kalwani", "otzuu-dldzs-avvu2-qwowd-hdj73-aocy7-lacgi-carzj-m6f2r-ffluy-fae"),
    ("Geodd Pvt Ltd", "eybf4-6t6bb-unfb2-h2hhn-rrfi2-cd2vs-phksn-jdmbn-i463m-4lzds-vqe"),
    ("GeoNodes LLC", "6sq7t-knkul-fko6h-xzvnf-ktbvr-jhx7r-hapzr-kjlek-whugy-zt6ip-xqe"),
    ("George Bassadone", "vegae-c4chr-aetfj-7gzuh-c23sx-u2paz-vmvbn-bcage-pu7lu-mptnn-eqe"),
    ("Giant Leaf, LLC", "wwdbq-xuqhf-eydzu-oyl7p-ga565-zm7s7-yrive-ozgsy-zzgh3-qwb3j-cae"),
    ("Honeycomb Capital (Pty) Ltd", "nmdd6-rouxw-55leh-wcbkn-kejit-njvje-p4s6e-v64d3-nlbjb-vipul-mae"),
    ("Iancu Aurel", "i7dto-bgkj2-xo5dx-cyrb7-zkk5y-q46eh-gz6iq-qkgyc-w4qte-scgtb-6ae"),
    ("Icaria Systems Pty Ltd", "ihbuj-erwnc-tkjux-tqtnv-zkoar-uniy2-sk2go-xfpkc-znbb4-seukm-wqe"),
    ("Illusions In Art (Pty) Ltd", "optdi-nwa4m-hly3k-6ua4n-sqyxf-yahvb-wps77-ddayn-r7zcz-edla5-7qe"),
    ("InfoObjects", "7ws2n-wqorv-vmo4m-5e222-n42c3-hk43s-ei3kp-4hpbn-xlkzo-jgv7i-tqe"),
    ("Ivanov Oleksandr", "ivf2y-crxj4-y6ewo-un35q-a7pum-wqmbw-pkepy-d6uew-bfmff-g5yxe-eae"),
    ("Karel Frank", "unqqg-no4b2-vbyad-ytik2-t3vly-3e57q-aje2t-sjb5l-bd4ke-chggn-uqe"),
    ("Kontrapunt (Pty) Ltd", "py2kr-ipr2p-ryh66-x3a3v-5ts6u-7rfhf-alkna-ueffh-hz5ox-lt6du-qqe"),
    ("Krishna Enterprises", "zy4m7-z5mhs-zfkpl-zlsjl-blrbx-mvvmq-5z4zu-mf7eq-hhv7o-ezfro-3ae"),
    ("Krzysztof Żelazko", "j2tnr-f5tmm-afnyl-762n7-o272x-ji2xi-bcpld-ihimy-fw52d-2zqov-xae"),
    ("Louise Velayo", "fnzev-s6xem-s2myy-rrxoa-2mpp6-oet33-pmnba-ajo75-qhfdw-esys7-7qe"),
    ("Lukas Helebrandt", "efem5-kmwaw-xose7-zzhgg-6bfif-twmcw-csg7a-lmqvn-wrdou-mjwlb-vqe"),
    ("Maksym Ishchenko", "4r6qy-tljxg-slziw-zoteo-pboxh-vlctz-hkv2d-7zior-u3pxm-mmuxb-cae"),
    ("Mariano Stoll", "s5nvr-ipdxf-xg6wd-ofacm-7tl4i-nwjzx-uulum-cugwb-kbpsa-wrsgs-cae"),
    ("Marvelous Web3", "7uioy-xitfw-yqcko-5gpya-3lpsw-dw7zt-dyyyf-wfqif-jvi76-fdbkg-cqe"),
    ("MB Patrankos šūvis", "4jjya-hlyyc-s766p-fd6gr-d6tvv-vo3ah-j5ptx-i73gw-mwgyd-rw6w2-rae"),
    ("MI Servers", "izmhk-lpjum-uo4oy-lviba-yctpc-arg4b-2ywim-vgoiu-gqaj2-gskmw-2qe"),
    ("ML SOLUTIONS LTD", "n6w7e-4cio3-an35h-hntwl-zzg4p-krqjk-yfmni-q7jiu-bage2-hef5b-pae"),
    ("Natalia Kulesha", "6ryfx-xszlo-xpvyj-b7vx6-m4erk-zwdkc-5lzfw-fty7k-arl66-uc3jk-nae"),
    ("Nataliia Nykyforak", "kf7dx-5wayj-3p2u4-yd4hf-m2en4-np75j-tta25-wqe7y-rlm6s-nqceb-7ae"),
    ("Neptune Partners", "4dibr-2alzr-h6kva-bvwn2-yqgsl-o577t-od46o-v275p-a2zov-tcw4f-eae"),
    ("Nikola Nikov", "kn4u4-unhbe-qwud4-ki6lq-o4try-6l2gv-yrxmg-vw6st-fmlss-nsztj-7qe"),
    ("NODAL CAPITAL", "kgfpq-4th36-lvnpn-ayygq-hikoq-dndag-vvafx-msvg5-aczmu-pkzsv-7ae"),
    ("NODAO", "g7dkt-aapqq-j3hqt-xtiys-pwapz-idulp-nwagd-zibqm-caxa4-gc23t-3qe"),
    ("NoviSystems, LLC", "hk7eo-22zam-kqmsx-dtfbj-k5i6f-jg65h-micpf-2cztc-t2eqk-efgvx-vqe"),
    ("OneSixtyTwo Digital Capital", "6nbcy-kprg6-ax3db-kh3cz-7jllk-oceyh-jznhs-riguq-fvk6z-6tsds-rqe"),
    ("Origin Game", "cgmhq-c4zja-yov4u-zeyao-64ua5-idlhb-ezcgr-cultv-3vqjs-dhwo7-rqe"),
    ("Paul Creasey", "xv5l6-677tb-f4ree-7cz2e-sawob-auqy2-23x7u-funsg-46eb2-sujic-jae"),
    ("Pindar Technology Limited", "r3yjn-kthmg-pfgmb-2fngg-5c7d7-t6kqg-wi37r-j7gy6-iee64-kjdja-jae"),
    ("Power Meta Corporation", "4fedi-eu6ue-nd7ts-vnof5-hzg66-hgzl7-liy5n-3otyp-h7ipw-owycg-uae"),
    ("Privoxy Solutions, LLC", "trxbq-wy5xi-3y27q-bkpaf-mhi2m-puexs-yatgt-nhwiy-dh6jy-rolw5-zqe"),
    ("Protocol16", "x7uok-pi537-itm37-unjn3-ewkze-kuetg-kptap-nuqak-auq7z-tn5ey-dqe"),
    ("Reist Telecom AG", "ma7dp-gz4tg-3c2wv-pgnsv-wna7u-czvhu-fpu47-t4dr6-gzxql-wr2m2-qae"),
    ("Richard Ma", "egb3e-rzi2e-vpsmm-akysp-l2owk-4dgst-b5hmg-xrkwa-cr3uk-zlzds-mae"),
    ("Rivonia Holdings LLC", "spp3m-vawt7-3gyh6-pjz5d-6zidf-up3qb-yte62-otexv-vfpqg-n6awf-lqe"),
    ("Rivram Inc", "ulyfm-vkxtj-o42dg-e4nam-l4tzf-37wci-ggntw-4ma7y-d267g-ywxi6-iae"),
    ("Serenity Lotus Limited", "2cfu2-qyug6-y4cme-lvj3c-6fs65-cbti4-ea6ig-nkaoj-fsbte-7n5gp-wae"),
    ("Starbase", "sixix-2nyqd-t2k2v-vlsyz-dssko-ls4hl-hyij4-y7mdp-ja6cj-nsmpf-yae"),
    ("Sygnum Bank", "6r5lw-l7db7-uwixn-iw5en-yy55y-ilbtq-e6gcv-g22r2-j3g6q-y37jk-jqe"),
    ("The Fenex Company LLC", "b7yyj-o7vc6-hdbzl-eggkm-bp2hg-3jmcv-5j5nn-t6zkq-ino4b-cvyde-yqe"),
    ("Tomahawk.vc", "ucjqj-jmbj3-rs4aq-ekzpw-ltjs3-zrcma-t6r3t-m5wxc-j5yrj-unwoj-mae"),
    ("Uvaca Labs LLC", "dodsd-rsjlg-sgekb-gr6mi-l6fck-tscwk-4jzgl-fwk4q-ncoyu-ulx53-aqe"),
    ("Virtual Hive, Ltd", "wdnqm-clqti-im5yf-iapio-avjom-kyppl-xuiza-oaz6z-smmts-52wyg-5ae"),
    ("Vladyslav Popov", "3oqw6-vmpk2-mlwlx-52z5x-e3p7u-fjlcw-yxc34-lf2zq-6ub2f-v63hk-lae"),
    ("Wancloud limited", "g2ax6-jrkmb-3zuh3-jibtb-q5xoq-njrgo-5utbc-j2o7g-zfq2w-yyhky-dqe"),
    ("Web3game", "64xe5-tx2s3-4gjmj-pnozr-fejw2-77y5y-rhcjk-glnmx-62brf-qin5q-pqe"),
    ("William Zelver", "usau7-upgoh-sg464-6qnso-lud42-nxho6-ith26-a2jhq-q5bgy-ajeou-4ae"),
    ("WMA Investments Limited", "7ne6c-3ahs2-76so4-te6hs-oq4mv-zhz4c-pqj2b-rxjmq-q56vn-tvpgj-2ae"),
    ("Wolkboer (Pty) Ltd", "mme7u-zxs3z-jq3un-fbaly-nllcz-toct2-l2kp3-larrb-gti4r-u2bmo-dae"),
    ("Zarety LLC", "glrjs-2dbzh-owbdd-fpp5e-eweoz-nsuto-e3jmk-tl42c-wem4f-qfpfa-qqe"),
    ("Zenith Code LLC", "pa5mu-yxsey-b4yrk-bodka-dhjnm-a3nx4-w2grw-3b766-ddr6e-nupu4-pqe"),
    ("Zondax AG", "hzqcb-iiagd-4erjo-qn7rq-syqro-zztl6-cpble-atnkd-2c6bg-bxjoa-qae"),
    ("ZTLC PTE LTD", "amsdj-4ss2k-wwcae-kroro-ippwx-lcro4-ysoha-uqlvc-3267j-vt3fy-yqe"),
    ("1G", "7k7b7-4pzhf-aivy6-y654t-uqyup-2auiz-ew2cm-4qkl4-nsl4v-bul5k-5qe"),
    ("A Dog's Boutique", "uvawj-nnt3i-4ch3a-hruy4-hyfjn-7owvl-wgkli-uq2hd-e6lkq-v4b2o-4ae"),
    ("Adam Dymecki", "n32q7-33lmk-m33tr-o5ltb-po6cb-tqqrr-2x6wp-pzhw7-ymizu-o3fyp-sqe"),
    ("Arjay LLC", "l2kri-jarwr-7whc4-pjdpn-n6hlb-45ltr-l6ghm-twttl-pcsvt-rynko-dqe"),
    ("Boolean Bit, LLC", "qdj4d-76lh3-w2q5i-kwjcd-643pq-pk42d-cziag-4hkau-35gib-m7s33-6qe"),
    ("Brener, Inc.", "2wxxr-qwylo-n7dhz-6co6m-iektd-vl7dn-ocvyc-xazaf-hbfxq-66spe-aae"),
    ("CRM52 Systems, LLC", "7nxxb-6qgm4-fftx3-xkwpj-sjrcm-tzmk5-dvuqk-l4ei4-3hvii-scwnj-tae"),
    ("DRMxTech Enterprises, LLC", "olgti-2hegv-ya7pd-ky2wt-of57j-tzs6q-ydrpy-hdxyy-cjnwx-ox5t4-3qe"),
    ("Fidgitville Ventures LLC", "ob633-g55bt-y6pu5-5iby6-jmcvi-oylqs-q6ahw-cvecq-5ckeh-m4wws-nae"),
    ("Finteck LLC", "x3zyd-pkcbf-5n3w2-n7uov-2qrbt-d3kfn-ojdd7-pxog5-vpqnt-6lex5-fqe"),
    ("Fritz Huie", "67gkg-gkgzz-g2ubz-3cc6h-jr3zm-twsii-7i325-r3gzr-kp2kh-dwxg6-pqe"),
    ("FUM Capital, LLC", "zgupw-boshs-mg6kz-2ciwm-upk7g-igjpn-75t3p-np6g4-47l7q-nuefa-6qe"),
    ("Goat, LLC", "p6fou-ngmgk-rxc6t-7ckzz-hojr2-kk6r3-xnlrk-ewzvu-g6xms-rfafz-zae"),
    ("Goodsir, LLC", "myrs2-bc6j6-mydpr-2jmli-l45mu-35ybt-c34mo-kjpve-zmaao-ajusy-nqe"),
    ("IC Pros", "srga3-cikqa-srnxx-rwejf-672jj-5o6qy-tuzsa-khds3-2ofjw-5gnew-mae"),
    ("Internet Computer Explorer", "i7v7g-cwjtl-gzd2s-nt2ko-4d5su-vh64h-bqu4h-rio26-tbaej-dtj6d-fae"),
    ("Jeffrey Schnettler", "5zqo2-omblo-i7knq-qyrfu-mjccn-tljyd-qslab-b7ukn-7tshi-pbeke-pae"),
    ("Jimmy Quach", "dzxyh-fo4sw-pxckk-kwqvc-xjten-3yqon-fm62b-2hz4s-raa4g-jzczg-iqe"),
    ("Jonathan Ziskind", "qcs4o-yswwp-7ozhg-m2ago-ytjyl-zlckb-raykw-fi5hl-cflyt-4beyv-zqe"),
    ("Joseph Stella", "f5wg2-kl4aq-cj4ym-nq7ul-i4kxy-t4x3x-yiyc5-3wlu6-opzwd-o5qes-hae"),
    ("Katerina Karapuz", "3teor-k2wwx-3xzqe-eufmv-zhysu-i4ml6-ka2qz-rgeei-oobbw-xka6i-vae"),
    ("KLAW Media, LLC", "pcwis-xaq3p-xvasz-5cfws-oelni-xs72v-acbwz-umxnq-nb2bc-ziyuu-sqe"),
    ("Krishna Sriram", "sajvr-l5iok-rj6c2-r76co-k2mde-d3kir-pnmjs-443f6-lxto4-pu55q-uae"),
    ("Lauren Dymecki Chickvara", "abscc-3lezh-oezci-5i3kz-pkwlc-ozz3r-5wv4n-htujn-rtajh-6cgyv-jae"),
    ("Luke Jacobson", "6dwst-olsa5-tagsr-jylgj-oicju-rf42e-o7job-gedre-fgoxt-7br62-jqe"),
    ("Marc Johnson", "xfvlz-qkgs4-xyeyx-lbke5-aarza-ovi52-wzpxl-3p27l-ov6ki-csxv6-qqe"),
    ("Mary Ren", "vdzyg-amckj-thvl5-bsn52-2elzd-drgii-ryh4c-izba3-xaehb-sohtd-aae"),
    ("Mika Properties, LLC", "3siog-htc6j-ed3wz-sguhu-2objz-g5qct-npoma-t3wwt-bd6wy-chwsi-4ae"),
    ("Michel Guerra", "4anlt-yam7x-eodmx-ik7mo-nl3kx-t35fj-52hfy-uv4jj-u2iea-ntg76-pqe"),
    ("Moon Block Ventures", "sdal5-w2c3d-p3buy-zieck-2wyuj-eu5bn-rkfe6-uuspi-o4n2b-gpei7-iae"),
    ("Mostly Wholesome, Inc", "ou3o7-akyjc-ldwd5-anyjn-l2buz-cwhbg-nehlc-abkde-qtc7w-fozdi-hae"),
    ("Paul Legato", "yr4eg-kwk3m-q44vj-ale35-2mtxk-5dyn7-vgppx-z6tcw-kzo4o-ezpm5-fqe"),
    ("Peggy Shafaghi", "72idx-a7c3y-nrcwc-lboj4-mmsas-sfdpm-gq23i-h2yuy-lykcj-vrxn2-jqe"),
    ("Philippe Chapparone", "2c4m6-25hos-qroi3-mk4aj-nog6s-zbzcv-ccpcc-cbv7s-sjy6p-bv3g5-fae"),
    ("Prayit Jain", "waj5k-wlyvv-jbj4n-vxwjm-dmkyg-uw2nl-ggojp-34kln-wgx3n-d7xih-5qe"),
    ("Rachel Dymecki", "chnsu-yaqt5-6osy5-au4zn-li6yu-nufmw-dewrt-utkiu-twd76-ujypw-rae"),
    ("Richard Suarez", "cmcjw-6c5ve-4zjnt-lipnl-2lp43-oh5wk-ewciz-xyvnv-m2rz5-hkm6a-hqe"),
    ("Ricky Sidhu", "q22bo-3uyqa-jvtpt-gapjk-pseor-esx4a-zyb74-vzea4-o7nx2-tafgq-hae"),
    ("Rishi Sachdev", "6tg64-cdfoh-kl35i-p6qti-sose3-746lr-jk5ex-phuvu-jfu3d-5svwa-7qe"),
    ("Rodney Zorilla", "bgprp-b2mnt-ci5in-57vuk-p7qvo-tj2tb-5w5su-qwenk-gbe77-mnuiq-sqe"),
    ("Ronnie Pellizzari", "wwxec-c2gd2-bu5on-ktpwz-z2ph3-vlr4p-m7ztf-6ck7r-nt3r4-fxbdq-mae"),
    ("Russell Ford", "p5jx4-lsrog-ep5o3-5uudg-opdg3-ur45z-xeefi-4ejfz-47t2k-riwfv-dae"),
    ("Scott Hallock", "i6sxi-fks25-viets-mboa7-3i23b-qeocf-e57qj-ar6vy-2mchu-xb5vp-aqe"),
    ("Shelburne Ventures, LLC", "a24zv-2ndbz-hqogc-ev63f-qxnpb-7ramd-usexl-ennaq-4om4k-sod6u-gae"),
    ("Staking Facilities", "niw4y-easue-l3qvz-sozsi-tfkvb-cxcx6-pzslg-5dqld-ooudp-hsuui-xae"),
    ("Starseed Technology, LLC", "ruxoj-jnqql-uau6o-xwrtb-ufde4-geddn-mnhni-wpew4-zhzi5-xjrxi-lqe"),
    ("Wolfhound LLC", "6mifr-stcqy-w5pzr-qpijh-jopft-p6jl3-n2sww-jhmzg-uzknn-hte4m-pae"),
    ("ZTLC PTE LTD (deprecated)", "xsrwt-tl3tk-n3aya-rafh3-ta6xu-eviw5-ae5dg-2f3bf-siaab-wdwdo-dqe"),
];

const SPAMMERS: &[&str] = &[
    "a28c30427beceb4a1cae7bad6145ad58767aa1364cd4466c1ff2ee2c70c40726",
    "62dd39780d34f2dc21eb680f99962659a6a0c2ccd9c68ec365962ae0eaf293f0",
];

const SNSES: &[(&str, &str)] = &[
    ("Alice", "oa5dz-haaaa-aaaaq-aaegq-cai"),
    ("Boom DAO", "xomae-vyaaa-aaaaq-aabhq-cai"),
    ("Catalyze", "umz53-fiaaa-aaaaq-aabmq-cai"),
    ("Cecil The Lion DAO", "jt5an-tqaaa-aaaaq-aaevq-cai"),
    ("Cycles Transfer Station", "igbbe-6yaaa-aaaaq-aadnq-cai"),
    ("DecideAI DAO", "xvj4b-paaaa-aaaaq-aabfa-cai"),
    ("DOGMI", "ni4my-zaaaa-aaaaq-aadra-cai"),
    ("DOLR AI", "6wcax-haaaa-aaaaq-aaava-cai"),
    ("Dragginz", "zqfso-syaaa-aaaaq-aaafq-cai"),
    ("ELNA AI", "gdnpl-daaaa-aaaaq-aacna-cai"),
    ("EstateDAO", "bmjwo-aqaaa-aaaaq-aac4a-cai"),
    ("FomoWell", "o3y74-5yaaa-aaaaq-aaeea-cai"),
    ("FuelEV", "nmkto-maaaa-aaaaq-aaemq-cai"),
    ("Gold DAO", "tr3th-kiaaa-aaaaq-aab6q-cai"),
    ("IC Explorer", "icx6s-lyaaa-aaaaq-aaeqa-cai"),
    ("ICFC", "detjl-sqaaa-aaaaq-aacqa-cai"),
    ("ICGhost", "4l7o7-uiaaa-aaaaq-aaa2q-cai"),
    ("ICLighthouse DAO", "hodlf-miaaa-aaaaq-aackq-cai"),
    ("ICPanda", "dwv6s-6aaaa-aaaaq-aacta-cai"),
    ("ICPCC DAO LLC", "lyqgk-ziaaa-aaaaq-aadeq-cai"),
    ("ICPEx", "lseuu-xyaaa-aaaaq-aaeya-cai"),
    ("ICPSwap", "cvzxu-kyaaa-aaaaq-aacvq-cai"),
    ("ICVC", "ntzq5-dyaaa-aaaaq-aadtq-cai"),
    ("Kinic", "74ncn-fqaaa-aaaaq-aaasa-cai"),
    ("KongSwap", "oypg6-faaaa-aaaaq-aadza-cai"),
    ("Motoko", "k34pm-nqaaa-aaaaq-aadca-cai"),
    ("Neutrinite", "eqsml-lyaaa-aaaaq-aacdq-cai"),
    ("NFID Wallet", "mpg2i-yyaaa-aaaaq-aaeka-cai"),
    ("Nuance", "rqch6-oaaaa-aaaaq-aabta-cai"),
    ("OpenChat", "2jvtu-yqaaa-aaaaq-aaama-cai"),
    ("ORIGYN", "lnxxh-yaaaa-aaaaq-aadha-cai"),
    ("Personal DAO", "iqrjl-hiaaa-aaaaq-aaeta-cai"),
    ("Seers", "rceqh-cqaaa-aaaaq-aabqa-cai"),
    ("Sneed", "fi3zi-fyaaa-aaaaq-aachq-cai"),
    ("SONIC", "qgj7v-3qaaa-aaaaq-aabwa-cai"),
    ("TRAX", "elxqo-raaaa-aaaaq-aacba-cai"),
    ("WaterNeuron", "jfnic-kaaaa-aaaaq-aadla-cai"),
    ("Yuku AI", "auadn-oqaaa-aaaaq-aacya-cai"),
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
