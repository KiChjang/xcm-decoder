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
        VersionedXcm::V2(xcm) => print_v2_xcm(&xcm),
        VersionedXcm::V3(xcm) => print_v3_xcm(&xcm),
    }

    Ok(())
}

fn print_v2_xcm(xcm: &xcm::v2::Xcm<()>) {
    let xcm_str = xcm
        .0
        .iter()
        .map(|inst| {
            use xcm::v2::Instruction::*;
            let inst_str = match inst {
                WithdrawAsset(_) => "WithdrawAsset",
                ReserveAssetDeposited(_) => "ReserveAssetDeposited",
                ReceiveTeleportedAsset(_) => "ReceiveTeleportedAsset",
                QueryResponse { .. } => "QueryResponse",
                TransferAsset { .. } => "TransferAsset",
                TransferReserveAsset { .. } => "TransferReserveAsset",
                Transact { .. } => "Transact",
                HrmpNewChannelOpenRequest { .. } => "HrmpNewChannelOpenRequest",
                HrmpChannelAccepted { .. } => "HrmpChannelAccepted",
                HrmpChannelClosing { .. } => "HrmpChannelClosing",
                ClearOrigin => "ClearOrigin",
                DescendOrigin(_) => "DescendOrigin",
                ReportError { .. } => "ReportError",
                DepositAsset { .. } => "DepositAsset",
                DepositReserveAsset { .. } => "DepositReserveAsset",
                ExchangeAsset { .. } => "ExchangeAsset",
                InitiateReserveWithdraw { .. } => "InitiateReserveWithdraw",
                InitiateTeleport { .. } => "InitiateTeleport",
                QueryHolding { .. } => "QueryHolding",
                BuyExecution { .. } => "BuyExecution",
                RefundSurplus => "RefundSurplus",
                SetErrorHandler(_) => "SetErrorHandler",
                SetAppendix(_) => "SetAppendix",
                ClearError => "ClearError",
                ClaimAsset { .. } => "ClaimAsset",
                Trap(_) => "Trap",
                SubscribeVersion { .. } => "SubscribeVersion",
                UnsubscribeVersion => "UnsubscribeVersion",
            };
            inst_str.to_string()
        })
        .fold("v2".to_string(), |mut acc, inst| {
            acc.push(',');
            acc.push_str(&inst);
            acc
        });

    println!("{}", xcm_str);
}

fn print_v3_xcm(xcm: &xcm::v3::Xcm<()>) {
    let xcm_str = xcm
        .0
        .iter()
        .map(|inst| {
            use xcm::v3::Instruction::*;
            let inst_str = match inst {
                WithdrawAsset(_) => "WithdrawAsset",
                ReserveAssetDeposited(_) => "ReserveAssetDeposited",
                ReceiveTeleportedAsset(_) => "ReceiveTeleportedAsset",
                QueryResponse { .. } => "QueryResponse",
                TransferAsset { .. } => "TransferAsset",
                TransferReserveAsset { .. } => "TransferReserveAsset",
                Transact { .. } => "Transact",
                HrmpNewChannelOpenRequest { .. } => "HrmpNewChannelOpenRequest",
                HrmpChannelAccepted { .. } => "HrmpChannelAccepted",
                HrmpChannelClosing { .. } => "HrmpChannelClosing",
                ClearOrigin => "ClearOrigin",
                DescendOrigin(_) => "DescendOrigin",
                ReportError { .. } => "ReportError",
                DepositAsset { .. } => "DepositAsset",
                DepositReserveAsset { .. } => "DepositReserveAsset",
                ExchangeAsset { .. } => "ExchangeAsset",
                InitiateReserveWithdraw { .. } => "InitiateReserveWithdraw",
                InitiateTeleport { .. } => "InitiateTeleport",
                ReportHolding { .. } => "ReportHolding",
                BuyExecution { .. } => "BuyExecution",
                RefundSurplus => "RefundSurplus",
                SetErrorHandler(_) => "SetErrorHandler",
                SetAppendix(_) => "SetAppendix",
                ClearError => "ClearError",
                ClaimAsset { .. } => "ClaimAsset",
                Trap(_) => "Trap",
                SubscribeVersion { .. } => "SubscribeVersion",
                UnsubscribeVersion => "UnsubscribeVersion",
                BurnAsset(_) => "BurnAsset",
                ExpectAsset(_) => "ExpectAsset",
                ExpectOrigin(_) => "ExpectOrigin",
                ExpectError(_) => "ExpectError",
                ExpectTransactStatus(_) => "ExpectTransactStatus",
                QueryPallet { .. } => "QueryPallet",
                ExpectPallet { .. } => "ExpectPallet",
                ReportTransactStatus(_) => "ReportTransactStatus",
                ClearTransactStatus => "ClearTransactStatus",
                UniversalOrigin(_) => "UniversalOrigin",
                ExportMessage { .. } => "ExportMessage",
                LockAsset { .. } => "LockAsset",
                UnlockAsset { .. } => "UnlockAsset",
                NoteUnlockable { .. } => "NoteUnlockable",
                RequestUnlock { .. } => "RequestUnlock",
                SetFeesMode { .. } => "SetFeesMode",
                SetTopic { .. } => "SetTopic",
                ClearTopic => "ClearTopic",
                AliasOrigin(_) => "AliasOrigin",
                UnpaidExecution { .. } => "UnpaidExecution",
            };
            inst_str.to_string()
        })
        .fold("v2:".to_string(), |mut acc, inst| {
            acc.push(',');
            acc.push_str(&inst);
            acc
        });

    println!("{}", xcm_str);
}
