use reliquary::network::GameCommand;
use reliquary::network::gen::command_id;
use tracing::warn;

use reliquary::network::gen::proto::ActivateFarmElementCsReq::ActivateFarmElementCsReq;
use reliquary::network::gen::proto::ActivateFarmElementScRsp::ActivateFarmElementScRsp;
use reliquary::network::gen::proto::ApplyFriendCsReq::ApplyFriendCsReq;
use reliquary::network::gen::proto::AvatarExpUpCsReq::AvatarExpUpCsReq;
use reliquary::network::gen::proto::AvatarExpUpScRsp::AvatarExpUpScRsp;
use reliquary::network::gen::proto::BattlePassInfoNotify::BattlePassInfoNotify;
use reliquary::network::gen::proto::BuyGoodsCsReq::BuyGoodsCsReq;
use reliquary::network::gen::proto::BuyGoodsScRsp::BuyGoodsScRsp;
use reliquary::network::gen::proto::ChallengeLineupNotify::ChallengeLineupNotify;
use reliquary::network::gen::proto::ChallengeSettleNotify::ChallengeSettleNotify;
use reliquary::network::gen::proto::ChangeLineupLeaderCsReq::ChangeLineupLeaderCsReq;
use reliquary::network::gen::proto::ChangeLineupLeaderScRsp::ChangeLineupLeaderScRsp;
use reliquary::network::gen::proto::ChessRogueCellUpdateNotify::ChessRogueCellUpdateNotify;
use reliquary::network::gen::proto::ChessRogueConfirmRollScRsp::ChessRogueConfirmRollScRsp;
use reliquary::network::gen::proto::ChessRogueNousEditDiceCsReq::ChessRogueNousEditDiceCsReq;
use reliquary::network::gen::proto::ChessRogueNousEditDiceScRsp::ChessRogueNousEditDiceScRsp;
use reliquary::network::gen::proto::ChessRogueReRollDiceCsReq::ChessRogueReRollDiceCsReq;
use reliquary::network::gen::proto::ChessRogueRollDiceCsReq::ChessRogueRollDiceCsReq;
use reliquary::network::gen::proto::ChessRogueRollDiceScRsp::ChessRogueRollDiceScRsp;
use reliquary::network::gen::proto::ChessRogueSelectCellCsReq::ChessRogueSelectCellCsReq;
use reliquary::network::gen::proto::ChessRogueSelectCellScRsp::ChessRogueSelectCellScRsp;
use reliquary::network::gen::proto::ChessRogueStartCsReq::ChessRogueStartCsReq;
use reliquary::network::gen::proto::ChessRogueStartScRsp::ChessRogueStartScRsp;
use reliquary::network::gen::proto::ChessRogueUpdateActionPointScNotify::ChessRogueUpdateActionPointScNotify;
use reliquary::network::gen::proto::ChessRogueUpdateAllowedSelectCellScNotify::ChessRogueUpdateAllowedSelectCellScNotify;
use reliquary::network::gen::proto::ChessRogueUpdateDiceInfoScNotify::ChessRogueUpdateDiceInfoScNotify;
use reliquary::network::gen::proto::ChessRogueUpdateDicePassiveAccumulateValueScNotify::ChessRogueUpdateDicePassiveAccumulateValueScNotify;
use reliquary::network::gen::proto::ChessRogueUpdateMoneyInfoScNotify::ChessRogueUpdateMoneyInfoScNotify;
use reliquary::network::gen::proto::ComposeItemCsReq::ComposeItemCsReq;
use reliquary::network::gen::proto::ComposeItemScRsp::ComposeItemScRsp;
use reliquary::network::gen::proto::ComposeSelectedRelicCsReq::ComposeSelectedRelicCsReq;
use reliquary::network::gen::proto::ComposeSelectedRelicScRsp::ComposeSelectedRelicScRsp;
use reliquary::network::gen::proto::DailyActiveInfoNotify::DailyActiveInfoNotify;
use reliquary::network::gen::proto::DeactivateFarmElementCsReq::DeactivateFarmElementCsReq;
use reliquary::network::gen::proto::DeactivateFarmElementScRsp::DeactivateFarmElementScRsp;
use reliquary::network::gen::proto::DeleteFriendCsReq::DeleteFriendCsReq;
use reliquary::network::gen::proto::DelMailCsReq::DelMailCsReq;
use reliquary::network::gen::proto::DelMailScRsp::DelMailScRsp;
use reliquary::network::gen::proto::DeployRotaterCsReq::DeployRotaterCsReq;
use reliquary::network::gen::proto::DeployRotaterScRsp::DeployRotaterScRsp;
use reliquary::network::gen::proto::DiscardRelicCsReq::DiscardRelicCsReq;
use reliquary::network::gen::proto::DoGachaCsReq::DoGachaCsReq;
use reliquary::network::gen::proto::DoGachaInRollShopCsReq::DoGachaInRollShopCsReq;
use reliquary::network::gen::proto::DoGachaInRollShopScRsp::DoGachaInRollShopScRsp;
use reliquary::network::gen::proto::DoGachaScRsp::DoGachaScRsp;
use reliquary::network::gen::proto::DressAvatarCsReq::DressAvatarCsReq;
use reliquary::network::gen::proto::DressRelicAvatarCsReq::DressRelicAvatarCsReq;
use reliquary::network::gen::proto::EnableRogueTalentCsReq::EnableRogueTalentCsReq;
use reliquary::network::gen::proto::EnableRogueTalentScRsp::EnableRogueTalentScRsp;
use reliquary::network::gen::proto::EnhanceRogueBuffCsReq::EnhanceRogueBuffCsReq;
use reliquary::network::gen::proto::EnhanceRogueBuffScRsp::EnhanceRogueBuffScRsp;
use reliquary::network::gen::proto::EnterMapRotationRegionCsReq::EnterMapRotationRegionCsReq;
use reliquary::network::gen::proto::EnterMapRotationRegionScRsp::EnterMapRotationRegionScRsp;
use reliquary::network::gen::proto::EnterRogueMapRoomCsReq::EnterRogueMapRoomCsReq;
use reliquary::network::gen::proto::EnterRogueMapRoomScRsp::EnterRogueMapRoomScRsp;
use reliquary::network::gen::proto::EnterSceneByServerScNotify::EnterSceneByServerScNotify;
use reliquary::network::gen::proto::EnterSceneCsReq::EnterSceneCsReq;
use reliquary::network::gen::proto::ExchangeGachaCeilingCsReq::ExchangeGachaCeilingCsReq;
use reliquary::network::gen::proto::ExchangeGachaCeilingScRsp::ExchangeGachaCeilingScRsp;
use reliquary::network::gen::proto::ExchangeHcoinCsReq::ExchangeHcoinCsReq;
use reliquary::network::gen::proto::ExchangeHcoinScRsp::ExchangeHcoinScRsp;
use reliquary::network::gen::proto::ExpUpEquipmentCsReq::ExpUpEquipmentCsReq;
use reliquary::network::gen::proto::ExpUpEquipmentScRsp::ExpUpEquipmentScRsp;
use reliquary::network::gen::proto::ExpUpRelicCsReq::ExpUpRelicCsReq;
use reliquary::network::gen::proto::ExpUpRelicScRsp::ExpUpRelicScRsp;
use reliquary::network::gen::proto::FinishRogueDialogueGroupCsReq::FinishRogueDialogueGroupCsReq;
use reliquary::network::gen::proto::GetActivityScheduleConfigScRsp::GetActivityScheduleConfigScRsp;
use reliquary::network::gen::proto::GetAllLineupDataScRsp::GetAllLineupDataScRsp;
use reliquary::network::gen::proto::GetArchiveDataScRsp::GetArchiveDataScRsp;
use reliquary::network::gen::proto::GetAvatarDataCsReq::GetAvatarDataCsReq;
use reliquary::network::gen::proto::GetAvatarDataScRsp::GetAvatarDataScRsp;
use reliquary::network::gen::proto::GetBagScRsp::GetBagScRsp;
use reliquary::network::gen::proto::GetBasicInfoScRsp::GetBasicInfoScRsp;
use reliquary::network::gen::proto::GetChallengeScRsp::GetChallengeScRsp;
use reliquary::network::gen::proto::GetChessRogueNousStoryInfoScRsp::GetChessRogueNousStoryInfoScRsp;
use reliquary::network::gen::proto::GetCurChallengeScRsp::GetCurChallengeScRsp;
use reliquary::network::gen::proto::GetCurLineupDataScRsp::GetCurLineupDataScRsp;
use reliquary::network::gen::proto::GetCurSceneInfoScRsp::GetCurSceneInfoScRsp;
use reliquary::network::gen::proto::GetDailyActiveInfoCsReq::GetDailyActiveInfoCsReq;
use reliquary::network::gen::proto::GetDailyActiveInfoScRsp::GetDailyActiveInfoScRsp;
use reliquary::network::gen::proto::GetEnteredSceneScRsp::GetEnteredSceneScRsp;
use reliquary::network::gen::proto::GetFarmStageGachaInfoCsReq::GetFarmStageGachaInfoCsReq;
use reliquary::network::gen::proto::GetFarmStageGachaInfoScRsp::GetFarmStageGachaInfoScRsp;
use reliquary::network::gen::proto::GetFirstTalkByPerformanceNpcCsReq::GetFirstTalkByPerformanceNpcCsReq;
use reliquary::network::gen::proto::GetFirstTalkByPerformanceNpcScRsp::GetFirstTalkByPerformanceNpcScRsp;
use reliquary::network::gen::proto::GetFirstTalkNpcCsReq::GetFirstTalkNpcCsReq;
use reliquary::network::gen::proto::GetFirstTalkNpcScRsp::GetFirstTalkNpcScRsp;
use reliquary::network::gen::proto::GetFriendApplyListInfoScRsp::GetFriendApplyListInfoScRsp;
use reliquary::network::gen::proto::GetFriendListInfoScRsp::GetFriendListInfoScRsp;
use reliquary::network::gen::proto::GetFriendLoginInfoScRsp::GetFriendLoginInfoScRsp;
use reliquary::network::gen::proto::GetFriendRecommendListInfoScRsp::GetFriendRecommendListInfoScRsp;
use reliquary::network::gen::proto::GetGachaCeilingCsReq::GetGachaCeilingCsReq;
use reliquary::network::gen::proto::GetGachaCeilingScRsp::GetGachaCeilingScRsp;
use reliquary::network::gen::proto::GetGachaInfoScRsp::GetGachaInfoScRsp;
use reliquary::network::gen::proto::GetHeroBasicTypeInfoScRsp::GetHeroBasicTypeInfoScRsp;
use reliquary::network::gen::proto::GetJukeboxDataCsReq::GetJukeboxDataCsReq;
use reliquary::network::gen::proto::GetJukeboxDataScRsp::GetJukeboxDataScRsp;
use reliquary::network::gen::proto::GetMailScRsp::GetMailScRsp;
use reliquary::network::gen::proto::GetMissionStatusCsReq::GetMissionStatusCsReq;
use reliquary::network::gen::proto::GetMissionStatusScRsp::GetMissionStatusScRsp;
use reliquary::network::gen::proto::GetNpcTakenRewardCsReq::GetNpcTakenRewardCsReq;
use reliquary::network::gen::proto::GetNpcTakenRewardScRsp::GetNpcTakenRewardScRsp;
use reliquary::network::gen::proto::GetPhoneDataCsReq::GetPhoneDataCsReq;
use reliquary::network::gen::proto::GetPhoneDataScRsp::GetPhoneDataScRsp;
use reliquary::network::gen::proto::GetPlayerBoardDataScRsp::GetPlayerBoardDataScRsp;
use reliquary::network::gen::proto::GetPlayerDetailInfoCsReq::GetPlayerDetailInfoCsReq;
use reliquary::network::gen::proto::GetPlayerDetailInfoScRsp::GetPlayerDetailInfoScRsp;
use reliquary::network::gen::proto::GetPrivateChatHistoryCsReq::GetPrivateChatHistoryCsReq;
use reliquary::network::gen::proto::GetPrivateChatHistoryScRsp::GetPrivateChatHistoryScRsp;
use reliquary::network::gen::proto::GetQuestDataCsReq::GetQuestDataCsReq;
use reliquary::network::gen::proto::GetQuestDataScRsp::GetQuestDataScRsp;
use reliquary::network::gen::proto::GetRogueBuffEnhanceInfoScRsp::GetRogueBuffEnhanceInfoScRsp;
use reliquary::network::gen::proto::GetRogueHandbookDataScRsp::GetRogueHandbookDataScRsp;
use reliquary::network::gen::proto::GetRogueInfoScRsp::GetRogueInfoScRsp;
use reliquary::network::gen::proto::GetRogueInitialScoreScRsp::GetRogueInitialScoreScRsp;
use reliquary::network::gen::proto::GetRogueScoreRewardInfoScRsp::GetRogueScoreRewardInfoScRsp;
use reliquary::network::gen::proto::GetRogueTalentInfoScRsp::GetRogueTalentInfoScRsp;
use reliquary::network::gen::proto::GetRollShopInfoCsReq::GetRollShopInfoCsReq;
use reliquary::network::gen::proto::GetRollShopInfoScRsp::GetRollShopInfoScRsp;
use reliquary::network::gen::proto::GetSceneMapInfoCsReq::GetSceneMapInfoCsReq;
use reliquary::network::gen::proto::GetSceneMapInfoScRsp::GetSceneMapInfoScRsp;
use reliquary::network::gen::proto::GetShopListCsReq::GetShopListCsReq;
use reliquary::network::gen::proto::GetShopListScRsp::GetShopListScRsp;
use reliquary::network::gen::proto::GetUnlockTeleportCsReq::GetUnlockTeleportCsReq;
use reliquary::network::gen::proto::GetUnlockTeleportScRsp::GetUnlockTeleportScRsp;
use reliquary::network::gen::proto::GroupStateChangeCsReq::GroupStateChangeCsReq;
use reliquary::network::gen::proto::GroupStateChangeScNotify::GroupStateChangeScNotify;
use reliquary::network::gen::proto::GroupStateChangeScRsp::GroupStateChangeScRsp;
use reliquary::network::gen::proto::HandleFriendCsReq::HandleFriendCsReq;
use reliquary::network::gen::proto::HandleFriendScRsp::HandleFriendScRsp;
use reliquary::network::gen::proto::HandleRogueCommonPendingActionCsReq::HandleRogueCommonPendingActionCsReq;
use reliquary::network::gen::proto::HandleRogueCommonPendingActionScRsp::HandleRogueCommonPendingActionScRsp;
use reliquary::network::gen::proto::InteractChargerCsReq::InteractChargerCsReq;
use reliquary::network::gen::proto::InteractChargerScRsp::InteractChargerScRsp;
use reliquary::network::gen::proto::InteractPropCsReq::InteractPropCsReq;
use reliquary::network::gen::proto::InteractPropScRsp::InteractPropScRsp;
use reliquary::network::gen::proto::JoinLineupCsReq::JoinLineupCsReq;
use reliquary::network::gen::proto::LeaveRogueScRsp::LeaveRogueScRsp;
use reliquary::network::gen::proto::LockEquipmentCsReq::LockEquipmentCsReq;
use reliquary::network::gen::proto::LockEquipmentScRsp::LockEquipmentScRsp;
use reliquary::network::gen::proto::LockRelicCsReq::LockRelicCsReq;
use reliquary::network::gen::proto::LockRelicScRsp::LockRelicScRsp;
use reliquary::network::gen::proto::MarkAvatarCsReq::MarkAvatarCsReq;
use reliquary::network::gen::proto::MarkAvatarScRsp::MarkAvatarScRsp;
use reliquary::network::gen::proto::MarkReadMailCsReq::MarkReadMailCsReq;
use reliquary::network::gen::proto::MarkReadMailScRsp::MarkReadMailScRsp;
use reliquary::network::gen::proto::NewMailScNotify::NewMailScNotify;
use reliquary::network::gen::proto::PickRogueAvatarCsReq::PickRogueAvatarCsReq;
use reliquary::network::gen::proto::PickRogueAvatarScRsp::PickRogueAvatarScRsp;
use reliquary::network::gen::proto::PlayBackGroundMusicCsReq::PlayBackGroundMusicCsReq;
use reliquary::network::gen::proto::PlayBackGroundMusicScRsp::PlayBackGroundMusicScRsp;
use reliquary::network::gen::proto::PlayerGetTokenCsReq::PlayerGetTokenCsReq;
use reliquary::network::gen::proto::PlayerGetTokenScRsp::PlayerGetTokenScRsp;
use reliquary::network::gen::proto::PlayerLoginScRsp::PlayerLoginScRsp;
use reliquary::network::gen::proto::PlayerSyncScNotify::PlayerSyncScNotify;
use reliquary::network::gen::proto::PromoteAvatarCsReq::PromoteAvatarCsReq;
use reliquary::network::gen::proto::PromoteEquipmentCsReq::PromoteEquipmentCsReq;
use reliquary::network::gen::proto::PVEBattleResultCsReq::PVEBattleResultCsReq;
use reliquary::network::gen::proto::PVEBattleResultScRsp::PVEBattleResultScRsp;
use reliquary::network::gen::proto::QuitLineupCsReq::QuitLineupCsReq;
use reliquary::network::gen::proto::RankUpAvatarCsReq::RankUpAvatarCsReq;
use reliquary::network::gen::proto::RankUpEquipmentCsReq::RankUpEquipmentCsReq;
use reliquary::network::gen::proto::ReEnterLastElementStageCsReq::ReEnterLastElementStageCsReq;
use reliquary::network::gen::proto::ReEnterLastElementStageScRsp::ReEnterLastElementStageScRsp;
use reliquary::network::gen::proto::RefreshTriggerByClientCsReq::RefreshTriggerByClientCsReq;
use reliquary::network::gen::proto::RefreshTriggerByClientScNotify::RefreshTriggerByClientScNotify;
use reliquary::network::gen::proto::RefreshTriggerByClientScRsp::RefreshTriggerByClientScRsp;
use reliquary::network::gen::proto::ReplaceLineupCsReq::ReplaceLineupCsReq;
use reliquary::network::gen::proto::ReserveStaminaExchangeCsReq::ReserveStaminaExchangeCsReq;
use reliquary::network::gen::proto::ReserveStaminaExchangeScRsp::ReserveStaminaExchangeScRsp;
use reliquary::network::gen::proto::RevcMsgScNotify::RevcMsgScNotify;
use reliquary::network::gen::proto::RogueNpcDisappearCsReq::RogueNpcDisappearCsReq;
use reliquary::network::gen::proto::RotateMapCsReq::RotateMapCsReq;
use reliquary::network::gen::proto::RotateMapScRsp::RotateMapScRsp;
use reliquary::network::gen::proto::SceneCastSkillCsReq::SceneCastSkillCsReq;
use reliquary::network::gen::proto::SceneCastSkillMpUpdateScNotify::SceneCastSkillMpUpdateScNotify;
use reliquary::network::gen::proto::SceneCastSkillScRsp::SceneCastSkillScRsp;
use reliquary::network::gen::proto::SceneEnterStageCsReq::SceneEnterStageCsReq;
use reliquary::network::gen::proto::SceneEnterStageScRsp::SceneEnterStageScRsp;
use reliquary::network::gen::proto::SceneEntityMoveCsReq::SceneEntityMoveCsReq;
use reliquary::network::gen::proto::SceneEntityMoveScNotify::SceneEntityMoveScNotify;
use reliquary::network::gen::proto::SceneEntityTeleportCsReq::SceneEntityTeleportCsReq;
use reliquary::network::gen::proto::SceneEntityTeleportScRsp::SceneEntityTeleportScRsp;
use reliquary::network::gen::proto::SceneGroupRefreshScNotify::SceneGroupRefreshScNotify;
use reliquary::network::gen::proto::ScenePlaneEventScNotify::ScenePlaneEventScNotify;
use reliquary::network::gen::proto::SearchPlayerCsReq::SearchPlayerCsReq;
use reliquary::network::gen::proto::SearchPlayerScRsp::SearchPlayerScRsp;
use reliquary::network::gen::proto::SelectChatBubbleCsReq::SelectChatBubbleCsReq;
use reliquary::network::gen::proto::SelectChatBubbleScRsp::SelectChatBubbleScRsp;
use reliquary::network::gen::proto::SelectPhoneThemeCsReq::SelectPhoneThemeCsReq;
use reliquary::network::gen::proto::SelectPhoneThemeScRsp::SelectPhoneThemeScRsp;
use reliquary::network::gen::proto::SelectRogueDialogueEventCsReq::SelectRogueDialogueEventCsReq;
use reliquary::network::gen::proto::SelectRogueDialogueEventScRsp::SelectRogueDialogueEventScRsp;
use reliquary::network::gen::proto::SellItemCsReq::SellItemCsReq;
use reliquary::network::gen::proto::SellItemScRsp::SellItemScRsp;
use reliquary::network::gen::proto::SendMsgCsReq::SendMsgCsReq;
use reliquary::network::gen::proto::SetClientPausedCsReq::SetClientPausedCsReq;
use reliquary::network::gen::proto::SetClientPausedScRsp::SetClientPausedScRsp;
use reliquary::network::gen::proto::SetFriendMarkCsReq::SetFriendMarkCsReq;
use reliquary::network::gen::proto::SetFriendMarkScRsp::SetFriendMarkScRsp;
use reliquary::network::gen::proto::SetGameplayBirthdayCsReq::SetGameplayBirthdayCsReq;
use reliquary::network::gen::proto::SetGameplayBirthdayScRsp::SetGameplayBirthdayScRsp;
use reliquary::network::gen::proto::SetHeadIconCsReq::SetHeadIconCsReq;
use reliquary::network::gen::proto::SetHeadIconScRsp::SetHeadIconScRsp;
use reliquary::network::gen::proto::SetHeroBasicTypeCsReq::SetHeroBasicTypeCsReq;
use reliquary::network::gen::proto::SetHeroBasicTypeScRsp::SetHeroBasicTypeScRsp;
use reliquary::network::gen::proto::SetLineupNameCsReq::SetLineupNameCsReq;
use reliquary::network::gen::proto::SetLineupNameScRsp::SetLineupNameScRsp;
use reliquary::network::gen::proto::SetNicknameCsReq::SetNicknameCsReq;
use reliquary::network::gen::proto::SetSignatureCsReq::SetSignatureCsReq;
use reliquary::network::gen::proto::SetSignatureScRsp::SetSignatureScRsp;
use reliquary::network::gen::proto::StaminaInfoScNotify::StaminaInfoScNotify;
use reliquary::network::gen::proto::StartChallengeCsReq::StartChallengeCsReq;
use reliquary::network::gen::proto::StartChallengeScRsp::StartChallengeScRsp;
use reliquary::network::gen::proto::StartCocoonStageCsReq::StartCocoonStageCsReq;
use reliquary::network::gen::proto::StartCocoonStageScRsp::StartCocoonStageScRsp;
use reliquary::network::gen::proto::StartRogueCsReq::StartRogueCsReq;
use reliquary::network::gen::proto::StartRogueScRsp::StartRogueScRsp;
use reliquary::network::gen::proto::SwapLineupCsReq::SwapLineupCsReq;
use reliquary::network::gen::proto::SwitchLineupIndexCsReq::SwitchLineupIndexCsReq;
use reliquary::network::gen::proto::SwitchLineupIndexScRsp::SwitchLineupIndexScRsp;
use reliquary::network::gen::proto::SyncApplyFriendScNotify::SyncApplyFriendScNotify;
use reliquary::network::gen::proto::SyncChessRogueNousValueScNotify::SyncChessRogueNousValueScNotify;
use reliquary::network::gen::proto::SyncClientResVersionCsReq::SyncClientResVersionCsReq;
use reliquary::network::gen::proto::SyncClientResVersionScRsp::SyncClientResVersionScRsp;
use reliquary::network::gen::proto::SyncDeleteFriendScNotify::SyncDeleteFriendScNotify;
use reliquary::network::gen::proto::SyncEntityBuffChangeListScNotify::SyncEntityBuffChangeListScNotify;
use reliquary::network::gen::proto::SyncHandleFriendScNotify::SyncHandleFriendScNotify;
use reliquary::network::gen::proto::SyncLineupNotify::SyncLineupNotify;
use reliquary::network::gen::proto::SyncRogueCommonActionResultScNotify::SyncRogueCommonActionResultScNotify;
use reliquary::network::gen::proto::SyncRogueCommonPendingActionScNotify::SyncRogueCommonPendingActionScNotify;
use reliquary::network::gen::proto::SyncRogueDialogueEventDataScNotify::SyncRogueDialogueEventDataScNotify;
use reliquary::network::gen::proto::SyncRogueFinishScNotify::SyncRogueFinishScNotify;
use reliquary::network::gen::proto::SyncRogueMapRoomScNotify::SyncRogueMapRoomScNotify;
use reliquary::network::gen::proto::SyncRogueStatusScNotify::SyncRogueStatusScNotify;
use reliquary::network::gen::proto::SyncRogueVirtualItemInfoScNotify::SyncRogueVirtualItemInfoScNotify;
use reliquary::network::gen::proto::TakeChallengeRewardCsReq::TakeChallengeRewardCsReq;
use reliquary::network::gen::proto::TakeChallengeRewardScRsp::TakeChallengeRewardScRsp;
use reliquary::network::gen::proto::TakeMailAttachmentCsReq::TakeMailAttachmentCsReq;
use reliquary::network::gen::proto::TakeMailAttachmentScRsp::TakeMailAttachmentScRsp;
use reliquary::network::gen::proto::TakeOffEquipmentCsReq::TakeOffEquipmentCsReq;
use reliquary::network::gen::proto::TakeOffRelicCsReq::TakeOffRelicCsReq;
use reliquary::network::gen::proto::TakePromotionRewardCsReq::TakePromotionRewardCsReq;
use reliquary::network::gen::proto::TakePromotionRewardScRsp::TakePromotionRewardScRsp;
use reliquary::network::gen::proto::TextJoinQueryCsReq::TextJoinQueryCsReq;
use reliquary::network::gen::proto::TextJoinQueryScRsp::TextJoinQueryScRsp;
use reliquary::network::gen::proto::UnlockBackGroundMusicCsReq::UnlockBackGroundMusicCsReq;
use reliquary::network::gen::proto::UnlockBackGroundMusicScRsp::UnlockBackGroundMusicScRsp;
use reliquary::network::gen::proto::UnlockChatBubbleScNotify::UnlockChatBubbleScNotify;
use reliquary::network::gen::proto::UnlockPhoneThemeScNotify::UnlockPhoneThemeScNotify;
use reliquary::network::gen::proto::UnlockSkilltreeCsReq::UnlockSkilltreeCsReq;
use reliquary::network::gen::proto::UnlockSkilltreeScRsp::UnlockSkilltreeScRsp;
use reliquary::network::gen::proto::UpdateEnergyScNotify::UpdateEnergyScNotify;
use reliquary::network::gen::proto::UpdateServerPrefsDataCsReq::UpdateServerPrefsDataCsReq;
use reliquary::network::gen::proto::UpdateServerPrefsDataScRsp::UpdateServerPrefsDataScRsp;
use reliquary::network::gen::proto::UseItemCsReq::UseItemCsReq;
use reliquary::network::gen::proto::UseItemScRsp::UseItemScRsp;

pub fn print_command(command: GameCommand) {
    match command.command_id {
        command_id::ActivateFarmElementCsReq => {
            let cmd = command.parse_proto::<ActivateFarmElementCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ActivateFarmElementScRsp => {
            let cmd = command.parse_proto::<ActivateFarmElementScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ApplyFriendCsReq => {
            let cmd = command.parse_proto::<ApplyFriendCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::AvatarExpUpCsReq => {
            let cmd = command.parse_proto::<AvatarExpUpCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::AvatarExpUpScRsp => {
            let cmd = command.parse_proto::<AvatarExpUpScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::BattlePassInfoNotify => {
            let cmd = command.parse_proto::<BattlePassInfoNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::BuyGoodsCsReq => {
            let cmd = command.parse_proto::<BuyGoodsCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::BuyGoodsScRsp => {
            let cmd = command.parse_proto::<BuyGoodsScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChallengeLineupNotify => {
            let cmd = command.parse_proto::<ChallengeLineupNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChallengeSettleNotify => {
            let cmd = command.parse_proto::<ChallengeSettleNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChangeLineupLeaderCsReq => {
            let cmd = command.parse_proto::<ChangeLineupLeaderCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChangeLineupLeaderScRsp => {
            let cmd = command.parse_proto::<ChangeLineupLeaderScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueCellUpdateNotify => {
            let cmd = command.parse_proto::<ChessRogueCellUpdateNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueConfirmRollScRsp => {
            let cmd = command.parse_proto::<ChessRogueConfirmRollScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueNousEditDiceCsReq => {
            let cmd = command.parse_proto::<ChessRogueNousEditDiceCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueNousEditDiceScRsp => {
            let cmd = command.parse_proto::<ChessRogueNousEditDiceScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueReRollDiceCsReq => {
            let cmd = command.parse_proto::<ChessRogueReRollDiceCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueRollDiceCsReq => {
            let cmd = command.parse_proto::<ChessRogueRollDiceCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueRollDiceScRsp => {
            let cmd = command.parse_proto::<ChessRogueRollDiceScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueSelectCellCsReq => {
            let cmd = command.parse_proto::<ChessRogueSelectCellCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueSelectCellScRsp => {
            let cmd = command.parse_proto::<ChessRogueSelectCellScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueStartCsReq => {
            let cmd = command.parse_proto::<ChessRogueStartCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueStartScRsp => {
            let cmd = command.parse_proto::<ChessRogueStartScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueUpdateActionPointScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateActionPointScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueUpdateAllowedSelectCellScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateAllowedSelectCellScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueUpdateDiceInfoScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateDiceInfoScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueUpdateDicePassiveAccumulateValueScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateDicePassiveAccumulateValueScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ChessRogueUpdateMoneyInfoScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateMoneyInfoScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ComposeItemCsReq => {
            let cmd = command.parse_proto::<ComposeItemCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ComposeItemScRsp => {
            let cmd = command.parse_proto::<ComposeItemScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ComposeSelectedRelicCsReq => {
            let cmd = command.parse_proto::<ComposeSelectedRelicCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ComposeSelectedRelicScRsp => {
            let cmd = command.parse_proto::<ComposeSelectedRelicScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DailyActiveInfoNotify => {
            let cmd = command.parse_proto::<DailyActiveInfoNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DeactivateFarmElementCsReq => {
            let cmd = command.parse_proto::<DeactivateFarmElementCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DeactivateFarmElementScRsp => {
            let cmd = command.parse_proto::<DeactivateFarmElementScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DeleteFriendCsReq => {
            let cmd = command.parse_proto::<DeleteFriendCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DelMailCsReq => {
            let cmd = command.parse_proto::<DelMailCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DelMailScRsp => {
            let cmd = command.parse_proto::<DelMailScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DeployRotaterCsReq => {
            let cmd = command.parse_proto::<DeployRotaterCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DeployRotaterScRsp => {
            let cmd = command.parse_proto::<DeployRotaterScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DiscardRelicCsReq => {
            let cmd = command.parse_proto::<DiscardRelicCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DoGachaCsReq => {
            let cmd = command.parse_proto::<DoGachaCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DoGachaInRollShopCsReq => {
            let cmd = command.parse_proto::<DoGachaInRollShopCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DoGachaInRollShopScRsp => {
            let cmd = command.parse_proto::<DoGachaInRollShopScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DoGachaScRsp => {
            let cmd = command.parse_proto::<DoGachaScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DressAvatarCsReq => {
            let cmd = command.parse_proto::<DressAvatarCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::DressRelicAvatarCsReq => {
            let cmd = command.parse_proto::<DressRelicAvatarCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnableRogueTalentCsReq => {
            let cmd = command.parse_proto::<EnableRogueTalentCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnableRogueTalentScRsp => {
            let cmd = command.parse_proto::<EnableRogueTalentScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnhanceRogueBuffCsReq => {
            let cmd = command.parse_proto::<EnhanceRogueBuffCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnhanceRogueBuffScRsp => {
            let cmd = command.parse_proto::<EnhanceRogueBuffScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnterMapRotationRegionCsReq => {
            let cmd = command.parse_proto::<EnterMapRotationRegionCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnterMapRotationRegionScRsp => {
            let cmd = command.parse_proto::<EnterMapRotationRegionScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnterRogueMapRoomCsReq => {
            let cmd = command.parse_proto::<EnterRogueMapRoomCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnterRogueMapRoomScRsp => {
            let cmd = command.parse_proto::<EnterRogueMapRoomScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnterSceneByServerScNotify => {
            let cmd = command.parse_proto::<EnterSceneByServerScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::EnterSceneCsReq => {
            let cmd = command.parse_proto::<EnterSceneCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ExchangeGachaCeilingCsReq => {
            let cmd = command.parse_proto::<ExchangeGachaCeilingCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ExchangeGachaCeilingScRsp => {
            let cmd = command.parse_proto::<ExchangeGachaCeilingScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ExchangeHcoinCsReq => {
            let cmd = command.parse_proto::<ExchangeHcoinCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ExchangeHcoinScRsp => {
            let cmd = command.parse_proto::<ExchangeHcoinScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ExpUpEquipmentCsReq => {
            let cmd = command.parse_proto::<ExpUpEquipmentCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ExpUpEquipmentScRsp => {
            let cmd = command.parse_proto::<ExpUpEquipmentScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ExpUpRelicCsReq => {
            let cmd = command.parse_proto::<ExpUpRelicCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ExpUpRelicScRsp => {
            let cmd = command.parse_proto::<ExpUpRelicScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::FinishRogueDialogueGroupCsReq => {
            let cmd = command.parse_proto::<FinishRogueDialogueGroupCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetActivityScheduleConfigScRsp => {
            let cmd = command.parse_proto::<GetActivityScheduleConfigScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetAllLineupDataScRsp => {
            let cmd = command.parse_proto::<GetAllLineupDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetArchiveDataScRsp => {
            let cmd = command.parse_proto::<GetArchiveDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetAvatarDataCsReq => {
            let cmd = command.parse_proto::<GetAvatarDataCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetAvatarDataScRsp => {
            let cmd = command.parse_proto::<GetAvatarDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetBagScRsp => {
            let cmd = command.parse_proto::<GetBagScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetBasicInfoScRsp => {
            let cmd = command.parse_proto::<GetBasicInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetChallengeScRsp => {
            let cmd = command.parse_proto::<GetChallengeScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetChessRogueNousStoryInfoScRsp => {
            let cmd = command.parse_proto::<GetChessRogueNousStoryInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetCurChallengeScRsp => {
            let cmd = command.parse_proto::<GetCurChallengeScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetCurLineupDataScRsp => {
            let cmd = command.parse_proto::<GetCurLineupDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetCurSceneInfoScRsp => {
            let cmd = command.parse_proto::<GetCurSceneInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetDailyActiveInfoCsReq => {
            let cmd = command.parse_proto::<GetDailyActiveInfoCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetDailyActiveInfoScRsp => {
            let cmd = command.parse_proto::<GetDailyActiveInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetEnteredSceneScRsp => {
            let cmd = command.parse_proto::<GetEnteredSceneScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFarmStageGachaInfoCsReq => {
            let cmd = command.parse_proto::<GetFarmStageGachaInfoCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFarmStageGachaInfoScRsp => {
            let cmd = command.parse_proto::<GetFarmStageGachaInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFirstTalkByPerformanceNpcCsReq => {
            let cmd = command.parse_proto::<GetFirstTalkByPerformanceNpcCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFirstTalkByPerformanceNpcScRsp => {
            let cmd = command.parse_proto::<GetFirstTalkByPerformanceNpcScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFirstTalkNpcCsReq => {
            let cmd = command.parse_proto::<GetFirstTalkNpcCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFirstTalkNpcScRsp => {
            let cmd = command.parse_proto::<GetFirstTalkNpcScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFriendApplyListInfoScRsp => {
            let cmd = command.parse_proto::<GetFriendApplyListInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFriendListInfoScRsp => {
            let cmd = command.parse_proto::<GetFriendListInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFriendLoginInfoScRsp => {
            let cmd = command.parse_proto::<GetFriendLoginInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetFriendRecommendListInfoScRsp => {
            let cmd = command.parse_proto::<GetFriendRecommendListInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetGachaCeilingCsReq => {
            let cmd = command.parse_proto::<GetGachaCeilingCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetGachaCeilingScRsp => {
            let cmd = command.parse_proto::<GetGachaCeilingScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetGachaInfoScRsp => {
            let cmd = command.parse_proto::<GetGachaInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetHeroBasicTypeInfoScRsp => {
            let cmd = command.parse_proto::<GetHeroBasicTypeInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetJukeboxDataCsReq => {
            let cmd = command.parse_proto::<GetJukeboxDataCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetJukeboxDataScRsp => {
            let cmd = command.parse_proto::<GetJukeboxDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetMailScRsp => {
            let cmd = command.parse_proto::<GetMailScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetMissionStatusCsReq => {
            let cmd = command.parse_proto::<GetMissionStatusCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetMissionStatusScRsp => {
            let cmd = command.parse_proto::<GetMissionStatusScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetNpcTakenRewardCsReq => {
            let cmd = command.parse_proto::<GetNpcTakenRewardCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetNpcTakenRewardScRsp => {
            let cmd = command.parse_proto::<GetNpcTakenRewardScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetPhoneDataCsReq => {
            let cmd = command.parse_proto::<GetPhoneDataCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetPhoneDataScRsp => {
            let cmd = command.parse_proto::<GetPhoneDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetPlayerBoardDataScRsp => {
            let cmd = command.parse_proto::<GetPlayerBoardDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetPlayerDetailInfoCsReq => {
            let cmd = command.parse_proto::<GetPlayerDetailInfoCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetPlayerDetailInfoScRsp => {
            let cmd = command.parse_proto::<GetPlayerDetailInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetPrivateChatHistoryCsReq => {
            let cmd = command.parse_proto::<GetPrivateChatHistoryCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetPrivateChatHistoryScRsp => {
            let cmd = command.parse_proto::<GetPrivateChatHistoryScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetQuestDataCsReq => {
            let cmd = command.parse_proto::<GetQuestDataCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetQuestDataScRsp => {
            let cmd = command.parse_proto::<GetQuestDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetRogueBuffEnhanceInfoScRsp => {
            let cmd = command.parse_proto::<GetRogueBuffEnhanceInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetRogueHandbookDataScRsp => {
            let cmd = command.parse_proto::<GetRogueHandbookDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetRogueInfoScRsp => {
            let cmd = command.parse_proto::<GetRogueInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetRogueInitialScoreScRsp => {
            let cmd = command.parse_proto::<GetRogueInitialScoreScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetRogueScoreRewardInfoScRsp => {
            let cmd = command.parse_proto::<GetRogueScoreRewardInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetRogueTalentInfoScRsp => {
            let cmd = command.parse_proto::<GetRogueTalentInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetRollShopInfoCsReq => {
            let cmd = command.parse_proto::<GetRollShopInfoCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetRollShopInfoScRsp => {
            let cmd = command.parse_proto::<GetRollShopInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetSceneMapInfoCsReq => {
            let cmd = command.parse_proto::<GetSceneMapInfoCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetSceneMapInfoScRsp => {
            let cmd = command.parse_proto::<GetSceneMapInfoScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetShopListCsReq => {
            let cmd = command.parse_proto::<GetShopListCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetShopListScRsp => {
            let cmd = command.parse_proto::<GetShopListScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetUnlockTeleportCsReq => {
            let cmd = command.parse_proto::<GetUnlockTeleportCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GetUnlockTeleportScRsp => {
            let cmd = command.parse_proto::<GetUnlockTeleportScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GroupStateChangeCsReq => {
            let cmd = command.parse_proto::<GroupStateChangeCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GroupStateChangeScNotify => {
            let cmd = command.parse_proto::<GroupStateChangeScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::GroupStateChangeScRsp => {
            let cmd = command.parse_proto::<GroupStateChangeScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::HandleFriendCsReq => {
            let cmd = command.parse_proto::<HandleFriendCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::HandleFriendScRsp => {
            let cmd = command.parse_proto::<HandleFriendScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::HandleRogueCommonPendingActionCsReq => {
            let cmd = command.parse_proto::<HandleRogueCommonPendingActionCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::HandleRogueCommonPendingActionScRsp => {
            let cmd = command.parse_proto::<HandleRogueCommonPendingActionScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::InteractChargerCsReq => {
            let cmd = command.parse_proto::<InteractChargerCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::InteractChargerScRsp => {
            let cmd = command.parse_proto::<InteractChargerScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::InteractPropCsReq => {
            let cmd = command.parse_proto::<InteractPropCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::InteractPropScRsp => {
            let cmd = command.parse_proto::<InteractPropScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::JoinLineupCsReq => {
            let cmd = command.parse_proto::<JoinLineupCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::LeaveRogueScRsp => {
            let cmd = command.parse_proto::<LeaveRogueScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::LockEquipmentCsReq => {
            let cmd = command.parse_proto::<LockEquipmentCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::LockEquipmentScRsp => {
            let cmd = command.parse_proto::<LockEquipmentScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::LockRelicCsReq => {
            let cmd = command.parse_proto::<LockRelicCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::LockRelicScRsp => {
            let cmd = command.parse_proto::<LockRelicScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::MarkAvatarCsReq => {
            let cmd = command.parse_proto::<MarkAvatarCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::MarkAvatarScRsp => {
            let cmd = command.parse_proto::<MarkAvatarScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::MarkReadMailCsReq => {
            let cmd = command.parse_proto::<MarkReadMailCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::MarkReadMailScRsp => {
            let cmd = command.parse_proto::<MarkReadMailScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::NewMailScNotify => {
            let cmd = command.parse_proto::<NewMailScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PickRogueAvatarCsReq => {
            let cmd = command.parse_proto::<PickRogueAvatarCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PickRogueAvatarScRsp => {
            let cmd = command.parse_proto::<PickRogueAvatarScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PlayBackGroundMusicCsReq => {
            let cmd = command.parse_proto::<PlayBackGroundMusicCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PlayBackGroundMusicScRsp => {
            let cmd = command.parse_proto::<PlayBackGroundMusicScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PlayerGetTokenCsReq => {
            let cmd = command.parse_proto::<PlayerGetTokenCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PlayerGetTokenScRsp => {
            let cmd = command.parse_proto::<PlayerGetTokenScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PlayerLoginScRsp => {
            let cmd = command.parse_proto::<PlayerLoginScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PlayerSyncScNotify => {
            let cmd = command.parse_proto::<PlayerSyncScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PromoteAvatarCsReq => {
            let cmd = command.parse_proto::<PromoteAvatarCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PromoteEquipmentCsReq => {
            let cmd = command.parse_proto::<PromoteEquipmentCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PVEBattleResultCsReq => {
            let cmd = command.parse_proto::<PVEBattleResultCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::PVEBattleResultScRsp => {
            let cmd = command.parse_proto::<PVEBattleResultScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::QuitLineupCsReq => {
            let cmd = command.parse_proto::<QuitLineupCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RankUpAvatarCsReq => {
            let cmd = command.parse_proto::<RankUpAvatarCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RankUpEquipmentCsReq => {
            let cmd = command.parse_proto::<RankUpEquipmentCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ReEnterLastElementStageCsReq => {
            let cmd = command.parse_proto::<ReEnterLastElementStageCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ReEnterLastElementStageScRsp => {
            let cmd = command.parse_proto::<ReEnterLastElementStageScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RefreshTriggerByClientCsReq => {
            let cmd = command.parse_proto::<RefreshTriggerByClientCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RefreshTriggerByClientScNotify => {
            let cmd = command.parse_proto::<RefreshTriggerByClientScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RefreshTriggerByClientScRsp => {
            let cmd = command.parse_proto::<RefreshTriggerByClientScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ReplaceLineupCsReq => {
            let cmd = command.parse_proto::<ReplaceLineupCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ReserveStaminaExchangeCsReq => {
            let cmd = command.parse_proto::<ReserveStaminaExchangeCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ReserveStaminaExchangeScRsp => {
            let cmd = command.parse_proto::<ReserveStaminaExchangeScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RevcMsgScNotify => {
            let cmd = command.parse_proto::<RevcMsgScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RogueNpcDisappearCsReq => {
            let cmd = command.parse_proto::<RogueNpcDisappearCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RotateMapCsReq => {
            let cmd = command.parse_proto::<RotateMapCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::RotateMapScRsp => {
            let cmd = command.parse_proto::<RotateMapScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneCastSkillCsReq => {
            let cmd = command.parse_proto::<SceneCastSkillCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneCastSkillMpUpdateScNotify => {
            let cmd = command.parse_proto::<SceneCastSkillMpUpdateScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneCastSkillScRsp => {
            let cmd = command.parse_proto::<SceneCastSkillScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneEnterStageCsReq => {
            let cmd = command.parse_proto::<SceneEnterStageCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneEnterStageScRsp => {
            let cmd = command.parse_proto::<SceneEnterStageScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneEntityMoveCsReq => {
            let cmd = command.parse_proto::<SceneEntityMoveCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneEntityMoveScNotify => {
            let cmd = command.parse_proto::<SceneEntityMoveScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneEntityTeleportCsReq => {
            let cmd = command.parse_proto::<SceneEntityTeleportCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneEntityTeleportScRsp => {
            let cmd = command.parse_proto::<SceneEntityTeleportScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SceneGroupRefreshScNotify => {
            let cmd = command.parse_proto::<SceneGroupRefreshScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::ScenePlaneEventScNotify => {
            let cmd = command.parse_proto::<ScenePlaneEventScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SearchPlayerCsReq => {
            let cmd = command.parse_proto::<SearchPlayerCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SearchPlayerScRsp => {
            let cmd = command.parse_proto::<SearchPlayerScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SelectChatBubbleCsReq => {
            let cmd = command.parse_proto::<SelectChatBubbleCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SelectChatBubbleScRsp => {
            let cmd = command.parse_proto::<SelectChatBubbleScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SelectPhoneThemeCsReq => {
            let cmd = command.parse_proto::<SelectPhoneThemeCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SelectPhoneThemeScRsp => {
            let cmd = command.parse_proto::<SelectPhoneThemeScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SelectRogueDialogueEventCsReq => {
            let cmd = command.parse_proto::<SelectRogueDialogueEventCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SelectRogueDialogueEventScRsp => {
            let cmd = command.parse_proto::<SelectRogueDialogueEventScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SellItemCsReq => {
            let cmd = command.parse_proto::<SellItemCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SellItemScRsp => {
            let cmd = command.parse_proto::<SellItemScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SendMsgCsReq => {
            let cmd = command.parse_proto::<SendMsgCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetClientPausedCsReq => {
            let cmd = command.parse_proto::<SetClientPausedCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetClientPausedScRsp => {
            let cmd = command.parse_proto::<SetClientPausedScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetFriendMarkCsReq => {
            let cmd = command.parse_proto::<SetFriendMarkCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetFriendMarkScRsp => {
            let cmd = command.parse_proto::<SetFriendMarkScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetGameplayBirthdayCsReq => {
            let cmd = command.parse_proto::<SetGameplayBirthdayCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetGameplayBirthdayScRsp => {
            let cmd = command.parse_proto::<SetGameplayBirthdayScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetHeadIconCsReq => {
            let cmd = command.parse_proto::<SetHeadIconCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetHeadIconScRsp => {
            let cmd = command.parse_proto::<SetHeadIconScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetHeroBasicTypeCsReq => {
            let cmd = command.parse_proto::<SetHeroBasicTypeCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetHeroBasicTypeScRsp => {
            let cmd = command.parse_proto::<SetHeroBasicTypeScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetLineupNameCsReq => {
            let cmd = command.parse_proto::<SetLineupNameCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetLineupNameScRsp => {
            let cmd = command.parse_proto::<SetLineupNameScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetNicknameCsReq => {
            let cmd = command.parse_proto::<SetNicknameCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetSignatureCsReq => {
            let cmd = command.parse_proto::<SetSignatureCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SetSignatureScRsp => {
            let cmd = command.parse_proto::<SetSignatureScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::StaminaInfoScNotify => {
            let cmd = command.parse_proto::<StaminaInfoScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::StartChallengeCsReq => {
            let cmd = command.parse_proto::<StartChallengeCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::StartChallengeScRsp => {
            let cmd = command.parse_proto::<StartChallengeScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::StartCocoonStageCsReq => {
            let cmd = command.parse_proto::<StartCocoonStageCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::StartCocoonStageScRsp => {
            let cmd = command.parse_proto::<StartCocoonStageScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::StartRogueCsReq => {
            let cmd = command.parse_proto::<StartRogueCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::StartRogueScRsp => {
            let cmd = command.parse_proto::<StartRogueScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SwapLineupCsReq => {
            let cmd = command.parse_proto::<SwapLineupCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SwitchLineupIndexCsReq => {
            let cmd = command.parse_proto::<SwitchLineupIndexCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SwitchLineupIndexScRsp => {
            let cmd = command.parse_proto::<SwitchLineupIndexScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncApplyFriendScNotify => {
            let cmd = command.parse_proto::<SyncApplyFriendScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncChessRogueNousValueScNotify => {
            let cmd = command.parse_proto::<SyncChessRogueNousValueScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncClientResVersionCsReq => {
            let cmd = command.parse_proto::<SyncClientResVersionCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncClientResVersionScRsp => {
            let cmd = command.parse_proto::<SyncClientResVersionScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncDeleteFriendScNotify => {
            let cmd = command.parse_proto::<SyncDeleteFriendScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncEntityBuffChangeListScNotify => {
            let cmd = command.parse_proto::<SyncEntityBuffChangeListScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncHandleFriendScNotify => {
            let cmd = command.parse_proto::<SyncHandleFriendScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncLineupNotify => {
            let cmd = command.parse_proto::<SyncLineupNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncRogueCommonActionResultScNotify => {
            let cmd = command.parse_proto::<SyncRogueCommonActionResultScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncRogueCommonPendingActionScNotify => {
            let cmd = command.parse_proto::<SyncRogueCommonPendingActionScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncRogueDialogueEventDataScNotify => {
            let cmd = command.parse_proto::<SyncRogueDialogueEventDataScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncRogueFinishScNotify => {
            let cmd = command.parse_proto::<SyncRogueFinishScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncRogueMapRoomScNotify => {
            let cmd = command.parse_proto::<SyncRogueMapRoomScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncRogueStatusScNotify => {
            let cmd = command.parse_proto::<SyncRogueStatusScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::SyncRogueVirtualItemInfoScNotify => {
            let cmd = command.parse_proto::<SyncRogueVirtualItemInfoScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TakeChallengeRewardCsReq => {
            let cmd = command.parse_proto::<TakeChallengeRewardCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TakeChallengeRewardScRsp => {
            let cmd = command.parse_proto::<TakeChallengeRewardScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TakeMailAttachmentCsReq => {
            let cmd = command.parse_proto::<TakeMailAttachmentCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TakeMailAttachmentScRsp => {
            let cmd = command.parse_proto::<TakeMailAttachmentScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TakeOffEquipmentCsReq => {
            let cmd = command.parse_proto::<TakeOffEquipmentCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TakeOffRelicCsReq => {
            let cmd = command.parse_proto::<TakeOffRelicCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TakePromotionRewardCsReq => {
            let cmd = command.parse_proto::<TakePromotionRewardCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TakePromotionRewardScRsp => {
            let cmd = command.parse_proto::<TakePromotionRewardScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TextJoinQueryCsReq => {
            let cmd = command.parse_proto::<TextJoinQueryCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::TextJoinQueryScRsp => {
            let cmd = command.parse_proto::<TextJoinQueryScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UnlockBackGroundMusicCsReq => {
            let cmd = command.parse_proto::<UnlockBackGroundMusicCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UnlockBackGroundMusicScRsp => {
            let cmd = command.parse_proto::<UnlockBackGroundMusicScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UnlockChatBubbleScNotify => {
            let cmd = command.parse_proto::<UnlockChatBubbleScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UnlockPhoneThemeScNotify => {
            let cmd = command.parse_proto::<UnlockPhoneThemeScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UnlockSkilltreeCsReq => {
            let cmd = command.parse_proto::<UnlockSkilltreeCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UnlockSkilltreeScRsp => {
            let cmd = command.parse_proto::<UnlockSkilltreeScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UpdateEnergyScNotify => {
            let cmd = command.parse_proto::<UpdateEnergyScNotify>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UpdateServerPrefsDataCsReq => {
            let cmd = command.parse_proto::<UpdateServerPrefsDataCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UpdateServerPrefsDataScRsp => {
            let cmd = command.parse_proto::<UpdateServerPrefsDataScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UseItemCsReq => {
            let cmd = command.parse_proto::<UseItemCsReq>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        command_id::UseItemScRsp => {
            let cmd = command.parse_proto::<UseItemScRsp>();
            match cmd {
                Ok(cmd) => {
                    println!("{:?}", cmd);
                }
                Err(error) => {
                    warn!(%error, "could not parse token command");
                }
            }
        }
        _ => None
    }
}
