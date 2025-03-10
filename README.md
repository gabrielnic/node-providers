# Node Providers

Every node provider from the IC network is documented here, from the
declaration document, to the proposals related to that node provider.

node providers rewards: https://hucfy-dyaaa-aaaam-ac37a-cai.icp0.io/

the registry canister is a state machine with only logs, so we need to create an index ourselves

-   use registry client to fetch all the records since the beginning
-   consume the data

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
