use std::any::Any;
use reliquary::network::GameCommand;
use reliquary::network::gen::command_id;

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
pub fn command_id_to_struct(command: GameCommand) -> Option<protobuf::Result<dyn Any>> {
    match command.command_id {
        command_id::ActivateFarmElementCsReq => {
            let cmd = command.parse_proto::<ActivateFarmElementCsReq>();
            Some(cmd)
        }
        command_id::ActivateFarmElementScRsp => {
            let cmd = command.parse_proto::<ActivateFarmElementScRsp>();
            Some(cmd)
        }
        command_id::ApplyFriendCsReq => {
            let cmd = command.parse_proto::<ApplyFriendCsReq>();
            Some(cmd)
        }
        command_id::AvatarExpUpCsReq => {
            let cmd = command.parse_proto::<AvatarExpUpCsReq>();
            Some(cmd)
        }
        command_id::AvatarExpUpScRsp => {
            let cmd = command.parse_proto::<AvatarExpUpScRsp>();
            Some(cmd)
        }
        command_id::BattlePassInfoNotify => {
            let cmd = command.parse_proto::<BattlePassInfoNotify>();
            Some(cmd)
        }
        command_id::BuyGoodsCsReq => {
            let cmd = command.parse_proto::<BuyGoodsCsReq>();
            Some(cmd)
        }
        command_id::BuyGoodsScRsp => {
            let cmd = command.parse_proto::<BuyGoodsScRsp>();
            Some(cmd)
        }
        command_id::ChallengeLineupNotify => {
            let cmd = command.parse_proto::<ChallengeLineupNotify>();
            Some(cmd)
        }
        command_id::ChallengeSettleNotify => {
            let cmd = command.parse_proto::<ChallengeSettleNotify>();
            Some(cmd)
        }
        command_id::ChangeLineupLeaderCsReq => {
            let cmd = command.parse_proto::<ChangeLineupLeaderCsReq>();
            Some(cmd)
        }
        command_id::ChangeLineupLeaderScRsp => {
            let cmd = command.parse_proto::<ChangeLineupLeaderScRsp>();
            Some(cmd)
        }
        command_id::ChessRogueCellUpdateNotify => {
            let cmd = command.parse_proto::<ChessRogueCellUpdateNotify>();
            Some(cmd)
        }
        command_id::ChessRogueConfirmRollScRsp => {
            let cmd = command.parse_proto::<ChessRogueConfirmRollScRsp>();
            Some(cmd)
        }
        command_id::ChessRogueNousEditDiceCsReq => {
            let cmd = command.parse_proto::<ChessRogueNousEditDiceCsReq>();
            Some(cmd)
        }
        command_id::ChessRogueNousEditDiceScRsp => {
            let cmd = command.parse_proto::<ChessRogueNousEditDiceScRsp>();
            Some(cmd)
        }
        command_id::ChessRogueReRollDiceCsReq => {
            let cmd = command.parse_proto::<ChessRogueReRollDiceCsReq>();
            Some(cmd)
        }
        command_id::ChessRogueRollDiceCsReq => {
            let cmd = command.parse_proto::<ChessRogueRollDiceCsReq>();
            Some(cmd)
        }
        command_id::ChessRogueRollDiceScRsp => {
            let cmd = command.parse_proto::<ChessRogueRollDiceScRsp>();
            Some(cmd)
        }
        command_id::ChessRogueSelectCellCsReq => {
            let cmd = command.parse_proto::<ChessRogueSelectCellCsReq>();
            Some(cmd)
        }
        command_id::ChessRogueSelectCellScRsp => {
            let cmd = command.parse_proto::<ChessRogueSelectCellScRsp>();
            Some(cmd)
        }
        command_id::ChessRogueStartCsReq => {
            let cmd = command.parse_proto::<ChessRogueStartCsReq>();
            Some(cmd)
        }
        command_id::ChessRogueStartScRsp => {
            let cmd = command.parse_proto::<ChessRogueStartScRsp>();
            Some(cmd)
        }
        command_id::ChessRogueUpdateActionPointScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateActionPointScNotify>();
            Some(cmd)
        }
        command_id::ChessRogueUpdateAllowedSelectCellScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateAllowedSelectCellScNotify>();
            Some(cmd)
        }
        command_id::ChessRogueUpdateDiceInfoScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateDiceInfoScNotify>();
            Some(cmd)
        }
        command_id::ChessRogueUpdateDicePassiveAccumulateValueScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateDicePassiveAccumulateValueScNotify>();
            Some(cmd)
        }
        command_id::ChessRogueUpdateMoneyInfoScNotify => {
            let cmd = command.parse_proto::<ChessRogueUpdateMoneyInfoScNotify>();
            Some(cmd)
        }
        command_id::ComposeItemCsReq => {
            let cmd = command.parse_proto::<ComposeItemCsReq>();
            Some(cmd)
        }
        command_id::ComposeItemScRsp => {
            let cmd = command.parse_proto::<ComposeItemScRsp>();
            Some(cmd)
        }
        command_id::ComposeSelectedRelicCsReq => {
            let cmd = command.parse_proto::<ComposeSelectedRelicCsReq>();
            Some(cmd)
        }
        command_id::ComposeSelectedRelicScRsp => {
            let cmd = command.parse_proto::<ComposeSelectedRelicScRsp>();
            Some(cmd)
        }
        command_id::DailyActiveInfoNotify => {
            let cmd = command.parse_proto::<DailyActiveInfoNotify>();
            Some(cmd)
        }
        command_id::DeactivateFarmElementCsReq => {
            let cmd = command.parse_proto::<DeactivateFarmElementCsReq>();
            Some(cmd)
        }
        command_id::DeactivateFarmElementScRsp => {
            let cmd = command.parse_proto::<DeactivateFarmElementScRsp>();
            Some(cmd)
        }
        command_id::DeleteFriendCsReq => {
            let cmd = command.parse_proto::<DeleteFriendCsReq>();
            Some(cmd)
        }
        command_id::DelMailCsReq => {
            let cmd = command.parse_proto::<DelMailCsReq>();
            Some(cmd)
        }
        command_id::DelMailScRsp => {
            let cmd = command.parse_proto::<DelMailScRsp>();
            Some(cmd)
        }
        command_id::DeployRotaterCsReq => {
            let cmd = command.parse_proto::<DeployRotaterCsReq>();
            Some(cmd)
        }
        command_id::DeployRotaterScRsp => {
            let cmd = command.parse_proto::<DeployRotaterScRsp>();
            Some(cmd)
        }
        command_id::DiscardRelicCsReq => {
            let cmd = command.parse_proto::<DiscardRelicCsReq>();
            Some(cmd)
        }
        command_id::DoGachaCsReq => {
            let cmd = command.parse_proto::<DoGachaCsReq>();
            Some(cmd)
        }
        command_id::DoGachaInRollShopCsReq => {
            let cmd = command.parse_proto::<DoGachaInRollShopCsReq>();
            Some(cmd)
        }
        command_id::DoGachaInRollShopScRsp => {
            let cmd = command.parse_proto::<DoGachaInRollShopScRsp>();
            Some(cmd)
        }
        command_id::DoGachaScRsp => {
            let cmd = command.parse_proto::<DoGachaScRsp>();
            Some(cmd)
        }
        command_id::DressAvatarCsReq => {
            let cmd = command.parse_proto::<DressAvatarCsReq>();
            Some(cmd)
        }
        command_id::DressRelicAvatarCsReq => {
            let cmd = command.parse_proto::<DressRelicAvatarCsReq>();
            Some(cmd)
        }
        command_id::EnableRogueTalentCsReq => {
            let cmd = command.parse_proto::<EnableRogueTalentCsReq>();
            Some(cmd)
        }
        command_id::EnableRogueTalentScRsp => {
            let cmd = command.parse_proto::<EnableRogueTalentScRsp>();
            Some(cmd)
        }
        command_id::EnhanceRogueBuffCsReq => {
            let cmd = command.parse_proto::<EnhanceRogueBuffCsReq>();
            Some(cmd)
        }
        command_id::EnhanceRogueBuffScRsp => {
            let cmd = command.parse_proto::<EnhanceRogueBuffScRsp>();
            Some(cmd)
        }
        command_id::EnterMapRotationRegionCsReq => {
            let cmd = command.parse_proto::<EnterMapRotationRegionCsReq>();
            Some(cmd)
        }
        command_id::EnterMapRotationRegionScRsp => {
            let cmd = command.parse_proto::<EnterMapRotationRegionScRsp>();
            Some(cmd)
        }
        command_id::EnterRogueMapRoomCsReq => {
            let cmd = command.parse_proto::<EnterRogueMapRoomCsReq>();
            Some(cmd)
        }
        command_id::EnterRogueMapRoomScRsp => {
            let cmd = command.parse_proto::<EnterRogueMapRoomScRsp>();
            Some(cmd)
        }
        command_id::EnterSceneByServerScNotify => {
            let cmd = command.parse_proto::<EnterSceneByServerScNotify>();
            Some(cmd)
        }
        command_id::EnterSceneCsReq => {
            let cmd = command.parse_proto::<EnterSceneCsReq>();
            Some(cmd)
        }
        command_id::ExchangeGachaCeilingCsReq => {
            let cmd = command.parse_proto::<ExchangeGachaCeilingCsReq>();
            Some(cmd)
        }
        command_id::ExchangeGachaCeilingScRsp => {
            let cmd = command.parse_proto::<ExchangeGachaCeilingScRsp>();
            Some(cmd)
        }
        command_id::ExchangeHcoinCsReq => {
            let cmd = command.parse_proto::<ExchangeHcoinCsReq>();
            Some(cmd)
        }
        command_id::ExchangeHcoinScRsp => {
            let cmd = command.parse_proto::<ExchangeHcoinScRsp>();
            Some(cmd)
        }
        command_id::ExpUpEquipmentCsReq => {
            let cmd = command.parse_proto::<ExpUpEquipmentCsReq>();
            Some(cmd)
        }
        command_id::ExpUpEquipmentScRsp => {
            let cmd = command.parse_proto::<ExpUpEquipmentScRsp>();
            Some(cmd)
        }
        command_id::ExpUpRelicCsReq => {
            let cmd = command.parse_proto::<ExpUpRelicCsReq>();
            Some(cmd)
        }
        command_id::ExpUpRelicScRsp => {
            let cmd = command.parse_proto::<ExpUpRelicScRsp>();
            Some(cmd)
        }
        command_id::FinishRogueDialogueGroupCsReq => {
            let cmd = command.parse_proto::<FinishRogueDialogueGroupCsReq>();
            Some(cmd)
        }
        command_id::GetActivityScheduleConfigScRsp => {
            let cmd = command.parse_proto::<GetActivityScheduleConfigScRsp>();
            Some(cmd)
        }
        command_id::GetAllLineupDataScRsp => {
            let cmd = command.parse_proto::<GetAllLineupDataScRsp>();
            Some(cmd)
        }
        command_id::GetArchiveDataScRsp => {
            let cmd = command.parse_proto::<GetArchiveDataScRsp>();
            Some(cmd)
        }
        command_id::GetAvatarDataCsReq => {
            let cmd = command.parse_proto::<GetAvatarDataCsReq>();
            Some(cmd)
        }
        command_id::GetAvatarDataScRsp => {
            let cmd = command.parse_proto::<GetAvatarDataScRsp>();
            Some(cmd)
        }
        command_id::GetBagScRsp => {
            let cmd = command.parse_proto::<GetBagScRsp>();
            Some(cmd)
        }
        command_id::GetBasicInfoScRsp => {
            let cmd = command.parse_proto::<GetBasicInfoScRsp>();
            Some(cmd)
        }
        command_id::GetChallengeScRsp => {
            let cmd = command.parse_proto::<GetChallengeScRsp>();
            Some(cmd)
        }
        command_id::GetChessRogueNousStoryInfoScRsp => {
            let cmd = command.parse_proto::<GetChessRogueNousStoryInfoScRsp>();
            Some(cmd)
        }
        command_id::GetCurChallengeScRsp => {
            let cmd = command.parse_proto::<GetCurChallengeScRsp>();
            Some(cmd)
        }
        command_id::GetCurLineupDataScRsp => {
            let cmd = command.parse_proto::<GetCurLineupDataScRsp>();
            Some(cmd)
        }
        command_id::GetCurSceneInfoScRsp => {
            let cmd = command.parse_proto::<GetCurSceneInfoScRsp>();
            Some(cmd)
        }
        command_id::GetDailyActiveInfoCsReq => {
            let cmd = command.parse_proto::<GetDailyActiveInfoCsReq>();
            Some(cmd)
        }
        command_id::GetDailyActiveInfoScRsp => {
            let cmd = command.parse_proto::<GetDailyActiveInfoScRsp>();
            Some(cmd)
        }
        command_id::GetEnteredSceneScRsp => {
            let cmd = command.parse_proto::<GetEnteredSceneScRsp>();
            Some(cmd)
        }
        command_id::GetFarmStageGachaInfoCsReq => {
            let cmd = command.parse_proto::<GetFarmStageGachaInfoCsReq>();
            Some(cmd)
        }
        command_id::GetFarmStageGachaInfoScRsp => {
            let cmd = command.parse_proto::<GetFarmStageGachaInfoScRsp>();
            Some(cmd)
        }
        command_id::GetFirstTalkByPerformanceNpcCsReq => {
            let cmd = command.parse_proto::<GetFirstTalkByPerformanceNpcCsReq>();
            Some(cmd)
        }
        command_id::GetFirstTalkByPerformanceNpcScRsp => {
            let cmd = command.parse_proto::<GetFirstTalkByPerformanceNpcScRsp>();
            Some(cmd)
        }
        command_id::GetFirstTalkNpcCsReq => {
            let cmd = command.parse_proto::<GetFirstTalkNpcCsReq>();
            Some(cmd)
        }
        command_id::GetFirstTalkNpcScRsp => {
            let cmd = command.parse_proto::<GetFirstTalkNpcScRsp>();
            Some(cmd)
        }
        command_id::GetFriendApplyListInfoScRsp => {
            let cmd = command.parse_proto::<GetFriendApplyListInfoScRsp>();
            Some(cmd)
        }
        command_id::GetFriendListInfoScRsp => {
            let cmd = command.parse_proto::<GetFriendListInfoScRsp>();
            Some(cmd)
        }
        command_id::GetFriendLoginInfoScRsp => {
            let cmd = command.parse_proto::<GetFriendLoginInfoScRsp>();
            Some(cmd)
        }
        command_id::GetFriendRecommendListInfoScRsp => {
            let cmd = command.parse_proto::<GetFriendRecommendListInfoScRsp>();
            Some(cmd)
        }
        command_id::GetGachaCeilingCsReq => {
            let cmd = command.parse_proto::<GetGachaCeilingCsReq>();
            Some(cmd)
        }
        command_id::GetGachaCeilingScRsp => {
            let cmd = command.parse_proto::<GetGachaCeilingScRsp>();
            Some(cmd)
        }
        command_id::GetGachaInfoScRsp => {
            let cmd = command.parse_proto::<GetGachaInfoScRsp>();
            Some(cmd)
        }
        command_id::GetHeroBasicTypeInfoScRsp => {
            let cmd = command.parse_proto::<GetHeroBasicTypeInfoScRsp>();
            Some(cmd)
        }
        command_id::GetJukeboxDataCsReq => {
            let cmd = command.parse_proto::<GetJukeboxDataCsReq>();
            Some(cmd)
        }
        command_id::GetJukeboxDataScRsp => {
            let cmd = command.parse_proto::<GetJukeboxDataScRsp>();
            Some(cmd)
        }
        command_id::GetMailScRsp => {
            let cmd = command.parse_proto::<GetMailScRsp>();
            Some(cmd)
        }
        command_id::GetMissionStatusCsReq => {
            let cmd = command.parse_proto::<GetMissionStatusCsReq>();
            Some(cmd)
        }
        command_id::GetMissionStatusScRsp => {
            let cmd = command.parse_proto::<GetMissionStatusScRsp>();
            Some(cmd)
        }
        command_id::GetNpcTakenRewardCsReq => {
            let cmd = command.parse_proto::<GetNpcTakenRewardCsReq>();
            Some(cmd)
        }
        command_id::GetNpcTakenRewardScRsp => {
            let cmd = command.parse_proto::<GetNpcTakenRewardScRsp>();
            Some(cmd)
        }
        command_id::GetPhoneDataCsReq => {
            let cmd = command.parse_proto::<GetPhoneDataCsReq>();
            Some(cmd)
        }
        command_id::GetPhoneDataScRsp => {
            let cmd = command.parse_proto::<GetPhoneDataScRsp>();
            Some(cmd)
        }
        command_id::GetPlayerBoardDataScRsp => {
            let cmd = command.parse_proto::<GetPlayerBoardDataScRsp>();
            Some(cmd)
        }
        command_id::GetPlayerDetailInfoCsReq => {
            let cmd = command.parse_proto::<GetPlayerDetailInfoCsReq>();
            Some(cmd)
        }
        command_id::GetPlayerDetailInfoScRsp => {
            let cmd = command.parse_proto::<GetPlayerDetailInfoScRsp>();
            Some(cmd)
        }
        command_id::GetPrivateChatHistoryCsReq => {
            let cmd = command.parse_proto::<GetPrivateChatHistoryCsReq>();
            Some(cmd)
        }
        command_id::GetPrivateChatHistoryScRsp => {
            let cmd = command.parse_proto::<GetPrivateChatHistoryScRsp>();
            Some(cmd)
        }
        command_id::GetQuestDataCsReq => {
            let cmd = command.parse_proto::<GetQuestDataCsReq>();
            Some(cmd)
        }
        command_id::GetQuestDataScRsp => {
            let cmd = command.parse_proto::<GetQuestDataScRsp>();
            Some(cmd)
        }
        command_id::GetRogueBuffEnhanceInfoScRsp => {
            let cmd = command.parse_proto::<GetRogueBuffEnhanceInfoScRsp>();
            Some(cmd)
        }
        command_id::GetRogueHandbookDataScRsp => {
            let cmd = command.parse_proto::<GetRogueHandbookDataScRsp>();
            Some(cmd)
        }
        command_id::GetRogueInfoScRsp => {
            let cmd = command.parse_proto::<GetRogueInfoScRsp>();
            Some(cmd)
        }
        command_id::GetRogueInitialScoreScRsp => {
            let cmd = command.parse_proto::<GetRogueInitialScoreScRsp>();
            Some(cmd)
        }
        command_id::GetRogueScoreRewardInfoScRsp => {
            let cmd = command.parse_proto::<GetRogueScoreRewardInfoScRsp>();
            Some(cmd)
        }
        command_id::GetRogueTalentInfoScRsp => {
            let cmd = command.parse_proto::<GetRogueTalentInfoScRsp>();
            Some(cmd)
        }
        command_id::GetRollShopInfoCsReq => {
            let cmd = command.parse_proto::<GetRollShopInfoCsReq>();
            Some(cmd)
        }
        command_id::GetRollShopInfoScRsp => {
            let cmd = command.parse_proto::<GetRollShopInfoScRsp>();
            Some(cmd)
        }
        command_id::GetSceneMapInfoCsReq => {
            let cmd = command.parse_proto::<GetSceneMapInfoCsReq>();
            Some(cmd)
        }
        command_id::GetSceneMapInfoScRsp => {
            let cmd = command.parse_proto::<GetSceneMapInfoScRsp>();
            Some(cmd)
        }
        command_id::GetShopListCsReq => {
            let cmd = command.parse_proto::<GetShopListCsReq>();
            Some(cmd)
        }
        command_id::GetShopListScRsp => {
            let cmd = command.parse_proto::<GetShopListScRsp>();
            Some(cmd)
        }
        command_id::GetUnlockTeleportCsReq => {
            let cmd = command.parse_proto::<GetUnlockTeleportCsReq>();
            Some(cmd)
        }
        command_id::GetUnlockTeleportScRsp => {
            let cmd = command.parse_proto::<GetUnlockTeleportScRsp>();
            Some(cmd)
        }
        command_id::GroupStateChangeCsReq => {
            let cmd = command.parse_proto::<GroupStateChangeCsReq>();
            Some(cmd)
        }
        command_id::GroupStateChangeScNotify => {
            let cmd = command.parse_proto::<GroupStateChangeScNotify>();
            Some(cmd)
        }
        command_id::GroupStateChangeScRsp => {
            let cmd = command.parse_proto::<GroupStateChangeScRsp>();
            Some(cmd)
        }
        command_id::HandleFriendCsReq => {
            let cmd = command.parse_proto::<HandleFriendCsReq>();
            Some(cmd)
        }
        command_id::HandleFriendScRsp => {
            let cmd = command.parse_proto::<HandleFriendScRsp>();
            Some(cmd)
        }
        command_id::HandleRogueCommonPendingActionCsReq => {
            let cmd = command.parse_proto::<HandleRogueCommonPendingActionCsReq>();
            Some(cmd)
        }
        command_id::HandleRogueCommonPendingActionScRsp => {
            let cmd = command.parse_proto::<HandleRogueCommonPendingActionScRsp>();
            Some(cmd)
        }
        command_id::InteractChargerCsReq => {
            let cmd = command.parse_proto::<InteractChargerCsReq>();
            Some(cmd)
        }
        command_id::InteractChargerScRsp => {
            let cmd = command.parse_proto::<InteractChargerScRsp>();
            Some(cmd)
        }
        command_id::InteractPropCsReq => {
            let cmd = command.parse_proto::<InteractPropCsReq>();
            Some(cmd)
        }
        command_id::InteractPropScRsp => {
            let cmd = command.parse_proto::<InteractPropScRsp>();
            Some(cmd)
        }
        command_id::JoinLineupCsReq => {
            let cmd = command.parse_proto::<JoinLineupCsReq>();
            Some(cmd)
        }
        command_id::LeaveRogueScRsp => {
            let cmd = command.parse_proto::<LeaveRogueScRsp>();
            Some(cmd)
        }
        command_id::LockEquipmentCsReq => {
            let cmd = command.parse_proto::<LockEquipmentCsReq>();
            Some(cmd)
        }
        command_id::LockEquipmentScRsp => {
            let cmd = command.parse_proto::<LockEquipmentScRsp>();
            Some(cmd)
        }
        command_id::LockRelicCsReq => {
            let cmd = command.parse_proto::<LockRelicCsReq>();
            Some(cmd)
        }
        command_id::LockRelicScRsp => {
            let cmd = command.parse_proto::<LockRelicScRsp>();
            Some(cmd)
        }
        command_id::MarkReadMailCsReq => {
            let cmd = command.parse_proto::<MarkReadMailCsReq>();
            Some(cmd)
        }
        command_id::MarkReadMailScRsp => {
            let cmd = command.parse_proto::<MarkReadMailScRsp>();
            Some(cmd)
        }
        command_id::NewMailScNotify => {
            let cmd = command.parse_proto::<NewMailScNotify>();
            Some(cmd)
        }
        command_id::PickRogueAvatarCsReq => {
            let cmd = command.parse_proto::<PickRogueAvatarCsReq>();
            Some(cmd)
        }
        command_id::PickRogueAvatarScRsp => {
            let cmd = command.parse_proto::<PickRogueAvatarScRsp>();
            Some(cmd)
        }
        command_id::PlayBackGroundMusicCsReq => {
            let cmd = command.parse_proto::<PlayBackGroundMusicCsReq>();
            Some(cmd)
        }
        command_id::PlayBackGroundMusicScRsp => {
            let cmd = command.parse_proto::<PlayBackGroundMusicScRsp>();
            Some(cmd)
        }
        command_id::PlayerGetTokenCsReq => {
            let cmd = command.parse_proto::<PlayerGetTokenCsReq>();
            Some(cmd)
        }
        command_id::PlayerGetTokenScRsp => {
            let cmd = command.parse_proto::<PlayerGetTokenScRsp>();
            Some(cmd)
        }
        command_id::PlayerLoginScRsp => {
            let cmd = command.parse_proto::<PlayerLoginScRsp>();
            Some(cmd)
        }
        command_id::PlayerSyncScNotify => {
            let cmd = command.parse_proto::<PlayerSyncScNotify>();
            Some(cmd)
        }
        command_id::PromoteAvatarCsReq => {
            let cmd = command.parse_proto::<PromoteAvatarCsReq>();
            Some(cmd)
        }
        command_id::PromoteEquipmentCsReq => {
            let cmd = command.parse_proto::<PromoteEquipmentCsReq>();
            Some(cmd)
        }
        command_id::PVEBattleResultCsReq => {
            let cmd = command.parse_proto::<PVEBattleResultCsReq>();
            Some(cmd)
        }
        command_id::PVEBattleResultScRsp => {
            let cmd = command.parse_proto::<PVEBattleResultScRsp>();
            Some(cmd)
        }
        command_id::QuitLineupCsReq => {
            let cmd = command.parse_proto::<QuitLineupCsReq>();
            Some(cmd)
        }
        command_id::RankUpAvatarCsReq => {
            let cmd = command.parse_proto::<RankUpAvatarCsReq>();
            Some(cmd)
        }
        command_id::RankUpEquipmentCsReq => {
            let cmd = command.parse_proto::<RankUpEquipmentCsReq>();
            Some(cmd)
        }
        command_id::ReEnterLastElementStageCsReq => {
            let cmd = command.parse_proto::<ReEnterLastElementStageCsReq>();
            Some(cmd)
        }
        command_id::ReEnterLastElementStageScRsp => {
            let cmd = command.parse_proto::<ReEnterLastElementStageScRsp>();
            Some(cmd)
        }
        command_id::RefreshTriggerByClientCsReq => {
            let cmd = command.parse_proto::<RefreshTriggerByClientCsReq>();
            Some(cmd)
        }
        command_id::RefreshTriggerByClientScNotify => {
            let cmd = command.parse_proto::<RefreshTriggerByClientScNotify>();
            Some(cmd)
        }
        command_id::RefreshTriggerByClientScRsp => {
            let cmd = command.parse_proto::<RefreshTriggerByClientScRsp>();
            Some(cmd)
        }
        command_id::ReplaceLineupCsReq => {
            let cmd = command.parse_proto::<ReplaceLineupCsReq>();
            Some(cmd)
        }
        command_id::ReserveStaminaExchangeCsReq => {
            let cmd = command.parse_proto::<ReserveStaminaExchangeCsReq>();
            Some(cmd)
        }
        command_id::ReserveStaminaExchangeScRsp => {
            let cmd = command.parse_proto::<ReserveStaminaExchangeScRsp>();
            Some(cmd)
        }
        command_id::RevcMsgScNotify => {
            let cmd = command.parse_proto::<RevcMsgScNotify>();
            Some(cmd)
        }
        command_id::RogueNpcDisappearCsReq => {
            let cmd = command.parse_proto::<RogueNpcDisappearCsReq>();
            Some(cmd)
        }
        command_id::RotateMapCsReq => {
            let cmd = command.parse_proto::<RotateMapCsReq>();
            Some(cmd)
        }
        command_id::RotateMapScRsp => {
            let cmd = command.parse_proto::<RotateMapScRsp>();
            Some(cmd)
        }
        command_id::SceneCastSkillCsReq => {
            let cmd = command.parse_proto::<SceneCastSkillCsReq>();
            Some(cmd)
        }
        command_id::SceneCastSkillMpUpdateScNotify => {
            let cmd = command.parse_proto::<SceneCastSkillMpUpdateScNotify>();
            Some(cmd)
        }
        command_id::SceneCastSkillScRsp => {
            let cmd = command.parse_proto::<SceneCastSkillScRsp>();
            Some(cmd)
        }
        command_id::SceneEnterStageCsReq => {
            let cmd = command.parse_proto::<SceneEnterStageCsReq>();
            Some(cmd)
        }
        command_id::SceneEnterStageScRsp => {
            let cmd = command.parse_proto::<SceneEnterStageScRsp>();
            Some(cmd)
        }
        command_id::SceneEntityMoveCsReq => {
            let cmd = command.parse_proto::<SceneEntityMoveCsReq>();
            Some(cmd)
        }
        command_id::SceneEntityMoveScNotify => {
            let cmd = command.parse_proto::<SceneEntityMoveScNotify>();
            Some(cmd)
        }
        command_id::SceneEntityTeleportCsReq => {
            let cmd = command.parse_proto::<SceneEntityTeleportCsReq>();
            Some(cmd)
        }
        command_id::SceneEntityTeleportScRsp => {
            let cmd = command.parse_proto::<SceneEntityTeleportScRsp>();
            Some(cmd)
        }
        command_id::SceneGroupRefreshScNotify => {
            let cmd = command.parse_proto::<SceneGroupRefreshScNotify>();
            Some(cmd)
        }
        command_id::ScenePlaneEventScNotify => {
            let cmd = command.parse_proto::<ScenePlaneEventScNotify>();
            Some(cmd)
        }
        command_id::SearchPlayerCsReq => {
            let cmd = command.parse_proto::<SearchPlayerCsReq>();
            Some(cmd)
        }
        command_id::SearchPlayerScRsp => {
            let cmd = command.parse_proto::<SearchPlayerScRsp>();
            Some(cmd)
        }
        command_id::SelectChatBubbleCsReq => {
            let cmd = command.parse_proto::<SelectChatBubbleCsReq>();
            Some(cmd)
        }
        command_id::SelectChatBubbleScRsp => {
            let cmd = command.parse_proto::<SelectChatBubbleScRsp>();
            Some(cmd)
        }
        command_id::SelectPhoneThemeCsReq => {
            let cmd = command.parse_proto::<SelectPhoneThemeCsReq>();
            Some(cmd)
        }
        command_id::SelectPhoneThemeScRsp => {
            let cmd = command.parse_proto::<SelectPhoneThemeScRsp>();
            Some(cmd)
        }
        command_id::SelectRogueDialogueEventCsReq => {
            let cmd = command.parse_proto::<SelectRogueDialogueEventCsReq>();
            Some(cmd)
        }
        command_id::SelectRogueDialogueEventScRsp => {
            let cmd = command.parse_proto::<SelectRogueDialogueEventScRsp>();
            Some(cmd)
        }
        command_id::SellItemCsReq => {
            let cmd = command.parse_proto::<SellItemCsReq>();
            Some(cmd)
        }
        command_id::SellItemScRsp => {
            let cmd = command.parse_proto::<SellItemScRsp>();
            Some(cmd)
        }
        command_id::SendMsgCsReq => {
            let cmd = command.parse_proto::<SendMsgCsReq>();
            Some(cmd)
        }
        command_id::SetClientPausedCsReq => {
            let cmd = command.parse_proto::<SetClientPausedCsReq>();
            Some(cmd)
        }
        command_id::SetClientPausedScRsp => {
            let cmd = command.parse_proto::<SetClientPausedScRsp>();
            Some(cmd)
        }
        command_id::SetFriendMarkCsReq => {
            let cmd = command.parse_proto::<SetFriendMarkCsReq>();
            Some(cmd)
        }
        command_id::SetFriendMarkScRsp => {
            let cmd = command.parse_proto::<SetFriendMarkScRsp>();
            Some(cmd)
        }
        command_id::SetGameplayBirthdayCsReq => {
            let cmd = command.parse_proto::<SetGameplayBirthdayCsReq>();
            Some(cmd)
        }
        command_id::SetGameplayBirthdayScRsp => {
            let cmd = command.parse_proto::<SetGameplayBirthdayScRsp>();
            Some(cmd)
        }
        command_id::SetHeadIconCsReq => {
            let cmd = command.parse_proto::<SetHeadIconCsReq>();
            Some(cmd)
        }
        command_id::SetHeadIconScRsp => {
            let cmd = command.parse_proto::<SetHeadIconScRsp>();
            Some(cmd)
        }
        command_id::SetHeroBasicTypeCsReq => {
            let cmd = command.parse_proto::<SetHeroBasicTypeCsReq>();
            Some(cmd)
        }
        command_id::SetHeroBasicTypeScRsp => {
            let cmd = command.parse_proto::<SetHeroBasicTypeScRsp>();
            Some(cmd)
        }
        command_id::SetLineupNameCsReq => {
            let cmd = command.parse_proto::<SetLineupNameCsReq>();
            Some(cmd)
        }
        command_id::SetLineupNameScRsp => {
            let cmd = command.parse_proto::<SetLineupNameScRsp>();
            Some(cmd)
        }
        command_id::SetNicknameCsReq => {
            let cmd = command.parse_proto::<SetNicknameCsReq>();
            Some(cmd)
        }
        command_id::SetSignatureCsReq => {
            let cmd = command.parse_proto::<SetSignatureCsReq>();
            Some(cmd)
        }
        command_id::SetSignatureScRsp => {
            let cmd = command.parse_proto::<SetSignatureScRsp>();
            Some(cmd)
        }
        command_id::StaminaInfoScNotify => {
            let cmd = command.parse_proto::<StaminaInfoScNotify>();
            Some(cmd)
        }
        command_id::StartChallengeCsReq => {
            let cmd = command.parse_proto::<StartChallengeCsReq>();
            Some(cmd)
        }
        command_id::StartChallengeScRsp => {
            let cmd = command.parse_proto::<StartChallengeScRsp>();
            Some(cmd)
        }
        command_id::StartCocoonStageCsReq => {
            let cmd = command.parse_proto::<StartCocoonStageCsReq>();
            Some(cmd)
        }
        command_id::StartCocoonStageScRsp => {
            let cmd = command.parse_proto::<StartCocoonStageScRsp>();
            Some(cmd)
        }
        command_id::StartRogueCsReq => {
            let cmd = command.parse_proto::<StartRogueCsReq>();
            Some(cmd)
        }
        command_id::StartRogueScRsp => {
            let cmd = command.parse_proto::<StartRogueScRsp>();
            Some(cmd)
        }
        command_id::SwapLineupCsReq => {
            let cmd = command.parse_proto::<SwapLineupCsReq>();
            Some(cmd)
        }
        command_id::SwitchLineupIndexCsReq => {
            let cmd = command.parse_proto::<SwitchLineupIndexCsReq>();
            Some(cmd)
        }
        command_id::SwitchLineupIndexScRsp => {
            let cmd = command.parse_proto::<SwitchLineupIndexScRsp>();
            Some(cmd)
        }
        command_id::SyncApplyFriendScNotify => {
            let cmd = command.parse_proto::<SyncApplyFriendScNotify>();
            Some(cmd)
        }
        command_id::SyncChessRogueNousValueScNotify => {
            let cmd = command.parse_proto::<SyncChessRogueNousValueScNotify>();
            Some(cmd)
        }
        command_id::SyncClientResVersionCsReq => {
            let cmd = command.parse_proto::<SyncClientResVersionCsReq>();
            Some(cmd)
        }
        command_id::SyncClientResVersionScRsp => {
            let cmd = command.parse_proto::<SyncClientResVersionScRsp>();
            Some(cmd)
        }
        command_id::SyncDeleteFriendScNotify => {
            let cmd = command.parse_proto::<SyncDeleteFriendScNotify>();
            Some(cmd)
        }
        command_id::SyncEntityBuffChangeListScNotify => {
            let cmd = command.parse_proto::<SyncEntityBuffChangeListScNotify>();
            Some(cmd)
        }
        command_id::SyncHandleFriendScNotify => {
            let cmd = command.parse_proto::<SyncHandleFriendScNotify>();
            Some(cmd)
        }
        command_id::SyncLineupNotify => {
            let cmd = command.parse_proto::<SyncLineupNotify>();
            Some(cmd)
        }
        command_id::SyncRogueCommonActionResultScNotify => {
            let cmd = command.parse_proto::<SyncRogueCommonActionResultScNotify>();
            Some(cmd)
        }
        command_id::SyncRogueCommonPendingActionScNotify => {
            let cmd = command.parse_proto::<SyncRogueCommonPendingActionScNotify>();
            Some(cmd)
        }
        command_id::SyncRogueDialogueEventDataScNotify => {
            let cmd = command.parse_proto::<SyncRogueDialogueEventDataScNotify>();
            Some(cmd)
        }
        command_id::SyncRogueFinishScNotify => {
            let cmd = command.parse_proto::<SyncRogueFinishScNotify>();
            Some(cmd)
        }
        command_id::SyncRogueMapRoomScNotify => {
            let cmd = command.parse_proto::<SyncRogueMapRoomScNotify>();
            Some(cmd)
        }
        command_id::SyncRogueStatusScNotify => {
            let cmd = command.parse_proto::<SyncRogueStatusScNotify>();
            Some(cmd)
        }
        command_id::SyncRogueVirtualItemInfoScNotify => {
            let cmd = command.parse_proto::<SyncRogueVirtualItemInfoScNotify>();
            Some(cmd)
        }
        command_id::TakeChallengeRewardCsReq => {
            let cmd = command.parse_proto::<TakeChallengeRewardCsReq>();
            Some(cmd)
        }
        command_id::TakeChallengeRewardScRsp => {
            let cmd = command.parse_proto::<TakeChallengeRewardScRsp>();
            Some(cmd)
        }
        command_id::TakeMailAttachmentCsReq => {
            let cmd = command.parse_proto::<TakeMailAttachmentCsReq>();
            Some(cmd)
        }
        command_id::TakeMailAttachmentScRsp => {
            let cmd = command.parse_proto::<TakeMailAttachmentScRsp>();
            Some(cmd)
        }
        command_id::TakeOffEquipmentCsReq => {
            let cmd = command.parse_proto::<TakeOffEquipmentCsReq>();
            Some(cmd)
        }
        command_id::TakeOffRelicCsReq => {
            let cmd = command.parse_proto::<TakeOffRelicCsReq>();
            Some(cmd)
        }
        command_id::TakePromotionRewardCsReq => {
            let cmd = command.parse_proto::<TakePromotionRewardCsReq>();
            Some(cmd)
        }
        command_id::TakePromotionRewardScRsp => {
            let cmd = command.parse_proto::<TakePromotionRewardScRsp>();
            Some(cmd)
        }
        command_id::TextJoinQueryCsReq => {
            let cmd = command.parse_proto::<TextJoinQueryCsReq>();
            Some(cmd)
        }
        command_id::TextJoinQueryScRsp => {
            let cmd = command.parse_proto::<TextJoinQueryScRsp>();
            Some(cmd)
        }
        command_id::UnlockBackGroundMusicCsReq => {
            let cmd = command.parse_proto::<UnlockBackGroundMusicCsReq>();
            Some(cmd)
        }
        command_id::UnlockBackGroundMusicScRsp => {
            let cmd = command.parse_proto::<UnlockBackGroundMusicScRsp>();
            Some(cmd)
        }
        command_id::UnlockChatBubbleScNotify => {
            let cmd = command.parse_proto::<UnlockChatBubbleScNotify>();
            Some(cmd)
        }
        command_id::UnlockPhoneThemeScNotify => {
            let cmd = command.parse_proto::<UnlockPhoneThemeScNotify>();
            Some(cmd)
        }
        command_id::UnlockSkilltreeCsReq => {
            let cmd = command.parse_proto::<UnlockSkilltreeCsReq>();
            Some(cmd)
        }
        command_id::UnlockSkilltreeScRsp => {
            let cmd = command.parse_proto::<UnlockSkilltreeScRsp>();
            Some(cmd)
        }
        command_id::UpdateEnergyScNotify => {
            let cmd = command.parse_proto::<UpdateEnergyScNotify>();
            Some(cmd)
        }
        command_id::UpdateServerPrefsDataCsReq => {
            let cmd = command.parse_proto::<UpdateServerPrefsDataCsReq>();
            Some(cmd)
        }
        command_id::UpdateServerPrefsDataScRsp => {
            let cmd = command.parse_proto::<UpdateServerPrefsDataScRsp>();
            Some(cmd)
        }
        command_id::UseItemCsReq => {
            let cmd = command.parse_proto::<UseItemCsReq>();
            Some(cmd)
        }
        command_id::UseItemScRsp => {
            let cmd = command.parse_proto::<UseItemScRsp>();
            Some(cmd)
        }
        _ => None
    }
}
