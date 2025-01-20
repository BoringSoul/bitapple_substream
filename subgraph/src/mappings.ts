import { Protobuf } from "as-proto/assembly";
import { Data as protoData } from "./pb/substreams/v1/program/Data";
import { WrapEvent, AssetInfo, MintEvent, BurnEvent, StakeEvent, UnstakeEvent } from "../generated/schema";
import { BigInt, log, crypto, Bytes} from "@graphprotocol/graph-ts";

export function handleTriggers(bytes: Uint8Array): void {
  const input = Protobuf.decode<protoData>(bytes, protoData.decode);
  for (let i = 0; i < input.wrapEventList.length; i++) {
    let wrapEvent = input.wrapEventList[i];
    let wrapEventEntity = new WrapEvent(wrapEvent.trxHash);
    wrapEventEntity.assetAccount = wrapEvent.assetAccount;
    wrapEventEntity.assetInfo = wrapEvent.assetInfo;
    wrapEventEntity.owner = wrapEvent.owner;
    wrapEventEntity.save();
  }
  
  
}
