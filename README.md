# Node Providers




```
{
    "name": "NP Name",
    "principal": "principal",
    "declaration-path": "/path",
    "declaration-wiki-hash": "hash",
    "declaration-real-hash": "hash",
    "identity-path": "/path",
    "identity-wiki-hash": "hash",
    "identity-real-hash": "hash",
    "wiki-link": "link",
    "dashboard-link": "link",
    "account id rewards": "account id",
    "rewards XDR": "191919",
    "region": ["Europe", "US"],
    "number of nodes": 0,
    "additional documents": [
        {
            "name": "dummy",
            "path": "dummy",
            "wiki-hash": "dummy",
            "real-hash": "dummy"
        },
        {
            "name": "dummy1",
            "path": "dummy1",
            "wiki-hash": "dummy",
            "real-hash": "dummy"
        }
    ]
}
```









Every node provider from the IC network is documented here, from the
declaration document, to the proposals related to that node provider.

node providers rewards: https://hucfy-dyaaa-aaaam-ac37a-cai.icp0.io/

- add a filter for country where the nodes are running

the registry canister is a state machine with only logs, so we need to create an index ourselves

-   use registry client to fetch all the records since the beginning
-   consume the data

-   every node provider should have a small description as to why they are uniquely qualified to run nodes

-   create a column for extra documents

-   create a category whether NP are Gen 1.5, or 2.

- documents can be either missing, incomplete, or present

https://github.com/dfinity/ic/blob/master/rs/registry/client/src/client.rs
https://github.com/dfinity/ic/blob/master/rs/registry/canister/canister/canister.rs
https://github.com/aviate-labs/agent-go/blob/main/clients/registry/client.go

Node ID
Node Operator ID
Node Provider ID

```
"5wzcz-mfner-c5wor-3liei-7mpao-vkult-zdds6-6n3rg-42ksv-6zzqt-vae":{"ipv6":"2401:7500:ff1:20:6801:6eff:feb9:35e0","ipv4":null,"node_operator_id":"ri4lg-drli2-d5zpi-tsseq-soivo-qrydm-cvjwd-dbmgz-al7fj-4al6w-iae","node_provider_id":"7uioy-xitfw-yqcko-5gpya-3lpsw-dw7zt-dyyyf-wfqif-jvi76-fdbkg-cqe","dc_id":"nd1","hostos_version_id":"2e269c77aa2f6b2353ddad6a4ac3d5ddcac196b1","domain":null},
```

node operator id is one datacenter or a rack

for each provider get their node operator principal id aaxec-cijpz-vzdwe-546fi-vahtf-s2awa-ldn7q-mzl2j-gmvhz-56zlq-mqe
https://dashboard.internetcomputer.org/proposal/125999
and find the correct proposl to ftech their node provider principal id

like this one
https://nns.ic0.app/proposal/?u=qoctq-giaaa-aaaaa-aaaea-cai&proposal=134939

we then link their account


We get all the node operator

```
ic-admin --nns-url https://ic0.app get-node-operator-list
```

We then get more information

```
enzo@arrakis:~/c/proposals$ ic-admin get-node-operator 6jel7-zolww-jy5ha-bwkva-fv5dx-nkolq-scb63-azu3w-ne3rr-ite7s-sae
Using NNS URLs: ["https://ic0.app/"]
Fetching the most recent value for key: node_operator_record_6jel7-zolww-jy5ha-bwkva-fv5dx-nkolq-scb63-azu3w-ne3rr-ite7s-sae
Most recent version is 38412. Value:
NodeOperator { node_operator_principal_id: 6jel7-zolww-jy5ha-bwkva-fv5dx-nkolq-scb63-azu3w-ne3rr-ite7s-sae, node_allowance: 0, node_provider_principal_id: fwnmn-zn7yt-5jaia-fkxlr-dzwyu-keguq-npfxq-mc72w-exeae-n5thj-oae, dc_id: "sc1", rewardable_nodes: {"type3.1": 3}, ipv6: None }
```

We then do the following to get their monthly rewards
```
enzo@arrakis:~/c/proposals$ ic-admin get-monthly-node-provider-rewards
```

https://dashboard.internetcomputer.org/canister/rwlgt-iiaaa-aaaaa-aaaaa-cai
registry canister


==> we do https://dashboard.internetcomputer.org/canister/rrkah-fqaaa-aaaaa-aaaaq-cai#list_node_provider_rewards if it's 28 bytes we need to compute the crc32 checksum and add it in front


## TODO
- [ ] table with all NP
- [ ] compute rewards per NP per month
- [ ] proposals linked to that NPs

https://globe.gl/

https://github.com/dfinity/dre/blob/038e782704d11740cb1378e98f6079c2a42e477c/rs/dre-canisters/node_status_canister/src/node_status_canister_backend/src/lib.rs
https://dfinity.github.io/dre/bazel/tips-and-tricks.html
https://hucfy-dyaaa-aaaam-ac37a-cai.icp0.io/providers/67gkg-gkgzz-g2ubz-3cc6h-jr3zm-twsii-7i325-r3gzr-kp2kh-dwxg6-pqe

https://wiki.internetcomputer.org/wiki/Node_Provider_NNS_proposals

https://github.com/dfinity/ic/blob/master/rs/registry/canister/proto/ic_registry_canister/pb/v1/registry.proto

Dashboard:
- https://etherscan.io/nodetracker

https://github.com/dfinity/ic/blob/master/packages/icrc-ledger-types/src/icrc/generic_value.rs#L16

-----------------

node provider id: eipr5-izbom-neyqh-s3ec2-52eww-cyfpg-qfomg-3dpwj-4pffh-34xcu-7qe
account id: a754940d8966382ca1043e91f60d426f75c73c6afabe6b7eee01d0439ccc3074

ks7ow-zvs7i-ratdk-azq34-zio2b-gbekj-qjicg-pfhp3-ovhgu-k5qql-dae
4c733e557d1a2c1c4dbe2d424095153a08c432d749f82370c2491cc0d85e1407 account id not where rewards are minted

-------------------

## Wiki Issues

NPs that are on the forum but do not exist on the dashboard:
- Altimist Ltd - https://wiki.internetcomputer.org/wiki/Altimist_Ltd
- Bitapp - https://wiki.internetcomputer.org/wiki/Bitapp
- Blockchain Literacy Foundation - https://wiki.internetcomputer.org/wiki/Blockchain_Literacy_Foundation
- CocoMango, LLC - https://wiki.internetcomputer.org/wiki/CocoMango_LLC

- Bianca manages nodes on her own, no company listed on self-declaration

- https://wiki.internetcomputer.org/wiki/Blockchain_Development_Labs_Inc does not have identity in pdf
- https://wiki.internetcomputer.org/wiki/Coplus_Limited does not have identity in pdf
- https://mywikis-wiki-media.s3.us-central-1.wasabisys.com/internetcomputer/Certified_Verification.pdf all the links are broken
- hashquark redirects to james wang - https://wiki.internetcomputer.org/wiki/User:James_Wang - neither are node provider listed on the network


- few node providers put jpg instead of pdf

- Katerina Karapuz redirects to https://wiki.internetcomputer.org/wiki/Karel_Frank instead of https://wiki.internetcomputer.org/wiki/Katerina_Karapuz

- there's a lot of south africans companies - create a column as to where the company is incorporated

- krishna entreprise has declaration and invoice as the same pdf - need to split them

- https://wiki.internetcomputer.org/wiki/Ludens_LLC does not exist as a node provider
- ML Solutions does not have a wiki page
- Peterer straight up have no documents https://wiki.internetcomputer.org/wiki/Michael_and_Dominik_Peterer and are not node providers either
- https://wiki.internetcomputer.org/wiki/Nova_LCC does not exist anymore

- novi systems has no identity document

need to figure out which ones are being on-boarded and off-boarded

- Origin Game redirects to https://wiki.internetcomputer.org/wiki/User:Soekawat

- Protocol16 https://wiki.internetcomputer.org/wiki/User:Protocol16 signed as Philip hurr
- Pindar does not have identity documents https://wiki.internetcomputer.org/wiki/PindarTechnologyLimited
- Power Meta https://wiki.internetcomputer.org/wiki/User:Powermetacorp identity docs are instructions on how to search for the company

- https://wiki.internetcomputer.org/wiki/Privoxy_Solutions,_LLC. does not have identity document
- quantum nodes is a not a NP anymore https://wiki.internetcomputer.org/wiki/Quantum_Node_Ltd.
- Rivram does not have identity documents https://wiki.internetcomputer.org/wiki/Rivram_Inc
- Stamper is not a NP anymore https://wiki.internetcomputer.org/wiki/Stamper_Co.,_Ltd.
- Timur does not exist anymore https://wiki.internetcomputer.org/wiki/Timur_Rakhimzhan
- does not have any more nodes https://wiki.internetcomputer.org/wiki/WMA_Investments_Limited
- does not have any more nodes https://wiki.internetcomputer.org/wiki/Zaboo_d.o.o.
- does not have any nodes https://wiki.internetcomputer.org/wiki/User:ZTLC_PTE_Ltd
