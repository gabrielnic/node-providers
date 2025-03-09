# Node Providers

Every node provider from the IC network is documented here, from the
self-declaration document, to the proposals related to that node provider.

node providers rewards: https://hucfy-dyaaa-aaaam-ac37a-cai.icp0.io/

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



## TODO
- [ ] table with all NP
- [ ] compute rewards per NP per month
- [ ] proposals linked to that NPs
