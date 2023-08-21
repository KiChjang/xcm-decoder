use clap::Parser;
use parity_scale_codec::Decode;
use xcm::prelude::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    bytes: String,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let bytes = hex::decode(cli.bytes).map_err(|e| e.to_string())?;

    let Ok(versioned_xcm) = VersionedXcm::<()>::decode(&mut &bytes[..])
    else { return Err("Cannot parse input as VersionedXcm".to_string()) };

    match versioned_xcm {
        VersionedXcm::V2(xcm) => println!("{}", stringize_v2_xcm(&xcm)),
        VersionedXcm::V3(xcm) => println!("{}", stringize_v3_xcm(&xcm)),
    }

    Ok(())
}

fn stringize_v2_xcm(xcm: &xcm::v2::Xcm<()>) -> String {
    xcm
        .0
        .iter()
        .map(|inst| {
            use xcm::v2::Instruction::*;
            match inst {
                WithdrawAsset(_) => "WithdrawAsset".to_string(),
                ReserveAssetDeposited(_) => "ReserveAssetDeposited".to_string(),
                ReceiveTeleportedAsset(_) => "ReceiveTeleportedAsset".to_string(),
                QueryResponse { .. } => "QueryResponse".to_string(),
                TransferAsset { .. } => "TransferAsset".to_string(),
                TransferReserveAsset { .. } => "TransferReserveAsset".to_string(),
                Transact { .. } => "Transact".to_string(),
                HrmpNewChannelOpenRequest { .. } => "HrmpNewChannelOpenRequest".to_string(),
                HrmpChannelAccepted { .. } => "HrmpChannelAccepted".to_string(),
                HrmpChannelClosing { .. } => "HrmpChannelClosing".to_string(),
                ClearOrigin => "ClearOrigin".to_string(),
                DescendOrigin(_) => "DescendOrigin".to_string(),
                ReportError { .. } => "ReportError".to_string(),
                DepositAsset { .. } => "DepositAsset".to_string(),
                DepositReserveAsset { .. } => "DepositReserveAsset".to_string(),
                ExchangeAsset { .. } => "ExchangeAsset".to_string(),
                InitiateReserveWithdraw { .. } => "InitiateReserveWithdraw".to_string(),
                InitiateTeleport { .. } => "InitiateTeleport".to_string(),
                QueryHolding { .. } => "QueryHolding".to_string(),
                BuyExecution { .. } => "BuyExecution".to_string(),
                RefundSurplus => "RefundSurplus".to_string(),
                SetErrorHandler(m) => format!("SetErrorHandler({})", stringize_v2_xcm(m)),
                SetAppendix(m) => format!("SetAppendix({})", stringize_v2_xcm(m)),
                ClearError => "ClearError".to_string(),
                ClaimAsset { .. } => "ClaimAsset".to_string(),
                Trap(_) => "Trap".to_string(),
                SubscribeVersion { .. } => "SubscribeVersion".to_string(),
                UnsubscribeVersion => "UnsubscribeVersion".to_string(),
            }
        })
        .reduce(|mut acc, inst| {
            acc.push(',');
            acc.push_str(&inst);
            acc
        })
        .unwrap_or("".to_string())
}

fn stringize_v3_xcm(xcm: &xcm::v3::Xcm<()>) -> String {
    xcm
        .0
        .iter()
        .map(|inst| {
            use xcm::v3::Instruction::*;
            match inst {
                WithdrawAsset(_) => "WithdrawAsset".to_string(),
                ReserveAssetDeposited(_) => "ReserveAssetDeposited".to_string(),
                ReceiveTeleportedAsset(_) => "ReceiveTeleportedAsset".to_string(),
                QueryResponse { .. } => "QueryResponse".to_string(),
                TransferAsset { .. } => "TransferAsset".to_string(),
                TransferReserveAsset { .. } => "TransferReserveAsset".to_string(),
                Transact { .. } => "Transact".to_string(),
                HrmpNewChannelOpenRequest { .. } => "HrmpNewChannelOpenRequest".to_string(),
                HrmpChannelAccepted { .. } => "HrmpChannelAccepted".to_string(),
                HrmpChannelClosing { .. } => "HrmpChannelClosing".to_string(),
                ClearOrigin => "ClearOrigin".to_string(),
                DescendOrigin(_) => "DescendOrigin".to_string(),
                ReportError { .. } => "ReportError".to_string(),
                DepositAsset { .. } => "DepositAsset".to_string(),
                DepositReserveAsset { .. } => "DepositReserveAsset".to_string(),
                ExchangeAsset { .. } => "ExchangeAsset".to_string(),
                InitiateReserveWithdraw { .. } => "InitiateReserveWithdraw".to_string(),
                InitiateTeleport { .. } => "InitiateTeleport".to_string(),
                ReportHolding { .. } => "ReportHolding".to_string(),
                BuyExecution { .. } => "BuyExecution".to_string(),
                RefundSurplus => "RefundSurplus".to_string(),
                SetErrorHandler(m) => format!("SetErrorHandle({})", stringize_v3_xcm(m)),
                SetAppendix(m) => format!("SetAppendix({})", stringize_v3_xcm(m)),
                ClearError => "ClearError".to_string(),
                ClaimAsset { .. } => "ClaimAsset".to_string(),
                Trap(_) => "Trap".to_string(),
                SubscribeVersion { .. } => "SubscribeVersion".to_string(),
                UnsubscribeVersion => "UnsubscribeVersion".to_string(),
                BurnAsset(_) => "BurnAsset".to_string(),
                ExpectAsset(_) => "ExpectAsset".to_string(),
                ExpectOrigin(_) => "ExpectOrigin".to_string(),
                ExpectError(_) => "ExpectError".to_string(),
                ExpectTransactStatus(_) => "ExpectTransactStatus".to_string(),
                QueryPallet { .. } => "QueryPallet".to_string(),
                ExpectPallet { .. } => "ExpectPallet".to_string(),
                ReportTransactStatus(_) => "ReportTransactStatus".to_string(),
                ClearTransactStatus => "ClearTransactStatus".to_string(),
                UniversalOrigin(_) => "UniversalOrigin".to_string(),
                ExportMessage { .. } => "ExportMessage".to_string(),
                LockAsset { .. } => "LockAsset".to_string(),
                UnlockAsset { .. } => "UnlockAsset".to_string(),
                NoteUnlockable { .. } => "NoteUnlockable".to_string(),
                RequestUnlock { .. } => "RequestUnlock".to_string(),
                SetFeesMode { .. } => "SetFeesMode".to_string(),
                SetTopic { .. } => "SetTopic".to_string(),
                ClearTopic => "ClearTopic".to_string(),
                AliasOrigin(_) => "AliasOrigin".to_string(),
                UnpaidExecution { .. } => "UnpaidExecution".to_string(),
            }
        })
        .reduce(|mut acc, inst| {
            acc.push(',');
            acc.push_str(&inst);
            acc
        })
        .unwrap_or("".to_string())
}
