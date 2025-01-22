import { Protobuf } from "as-proto/assembly";
import { Data as protoData } from "./pb/substreams/v1/program/Data";
import { WrapEvent, MintEvent, BurnEvent, StakeEvent, UnstakeEvent } from "../generated/schema";
import { Asset } from "./pb/substreams/v1/program/Asset";

export function handleTriggers(bytes: Uint8Array): void {
  const input = Protobuf.decode<protoData>(bytes, protoData.decode);
  for (let i = 0; i < input.wrapEventList.length; i++) {
    let wrapEvent = input.wrapEventList[i];
    let wrapEventEntity = new WrapEvent(wrapEvent.trxHash);
    wrapEventEntity.assetAccount = wrapEvent.assetAccount;
    wrapEventEntity.assetInfo = "";  // 设置默认值
    if(wrapEvent.assetInfo) { 
      const assetInfo = wrapEvent.assetInfo!;
      wrapEventEntity.assetInfo = `{owner:"${assetInfo.owner}",` +
        `supplyNo:${assetInfo.supplyNo},` +
        `startTime:${assetInfo.startTime},` +
        `mintAccount:"${assetInfo.mintAccount}",` +
        `tokenAccount:"${assetInfo.tokenAccount}",` +
        `assets:[${assetInfo.assets.map((a:Asset) => 
          `{"tokenAddress":"${a.tokenAddress}","amount":${a.amount}}`
        ).join(',')}]}`;
    }
    wrapEventEntity.owner = wrapEvent.owner;
    wrapEventEntity.save();
  }
  
  // 处理 Mint 事件
  for (let i = 0; i < input.mintEventList.length; i++) {
    let mintEvent = input.mintEventList[i];
    let mintEventEntity = new MintEvent(mintEvent.trxHash);
    mintEventEntity.assetAccount = mintEvent.assetAccount;
    mintEventEntity.mintAccount = mintEvent.mintAccount;
    mintEventEntity.tokenAccount = mintEvent.tokenAccount;
    mintEventEntity.save();
  }

  // 处理 Burn 事件
  for (let i = 0; i < input.burnEventList.length; i++) {
    let burnEvent = input.burnEventList[i];
    let burnEventEntity = new BurnEvent(burnEvent.trxHash);
    burnEventEntity.assetAccount = burnEvent.assetAccount;
    burnEventEntity.mintAccount = burnEvent.mintAccount;
    burnEventEntity.tokenAccount = burnEvent.tokenAccount;
    burnEventEntity.save();
  }

  // 处理 Stake 事件
  for (let i = 0; i < input.stakeEventList.length; i++) {
    let stakeEvent = input.stakeEventList[i];
    let stakeEventEntity = new StakeEvent(stakeEvent.trxHash);
    stakeEventEntity.assetAccount = stakeEvent.assetAccount;
    stakeEventEntity.stakeAccount = stakeEvent.stakeAccount;
    stakeEventEntity.mintAccount = stakeEvent.mintAccount;
    stakeEventEntity.ownerAccount = stakeEvent.ownerAccount;
    stakeEventEntity.authorityAccount = stakeEvent.authorityAccount;
    stakeEventEntity.ownerTokenAccount = stakeEvent.ownerTokenAccount;
    stakeEventEntity.stakerTokenAccount = stakeEvent.stakerTokenAccount;
    stakeEventEntity.save();
  }

  // 处理 Unstake 事件
  for (let i = 0; i < input.unstakeEventList.length; i++) {
    let unstakeEvent = input.unstakeEventList[i];
    let unstakeEventEntity = new UnstakeEvent(unstakeEvent.trxHash);
    unstakeEventEntity.assetAccount = unstakeEvent.assetAccount;
    unstakeEventEntity.stakeAccount = unstakeEvent.stakeAccount;
    unstakeEventEntity.mintAccount = unstakeEvent.mintAccount;
    unstakeEventEntity.ownerAccount = unstakeEvent.ownerAccount;
    unstakeEventEntity.authorityAccount = unstakeEvent.authorityAccount;
    unstakeEventEntity.ownerTokenAccount = unstakeEvent.ownerTokenAccount;
    unstakeEventEntity.stakerTokenAccount = unstakeEvent.stakerTokenAccount;
    unstakeEventEntity.save();
  }
}
