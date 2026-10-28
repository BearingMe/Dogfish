Scriptname DogfishNativeBridge Hidden
{Global native Papyrus interface bridging Creation Engine events to the dogfish_integration.dll runtime.}

; ============================================================================
; Native Function Declarations (Implemented in dogfish-integration cdylib)
; ============================================================================

; Claim a territorial holding (0..14).
; Return values:
;   0 = Success (ownership transferred, crime gold cleared)
;   1 = Invalid holding ID
;   2 = Holding not adjacent (staging camp required)
;   3 = Holding already owned by Player
;  99 = Adapter uninitialized
Int Function ClaimLocation(Int locationID, Bool hasStagingCamp = true) Global Native

; Query the current owning faction of a holding (0..14).
; Return values:
;   0 = Player Faction
;   1 = Imperial Legion
;   2 = Stormcloak Rebellion
;   3 = Neutral Local Authorities
;  -1 = Invalid ID or error
Int Function GetHoldingOwner(Int locationID) Global Native

; Query the current national treasury balance in gold.
Int Function GetTreasuryGold() Global Native

; Deposit an amount of gold into the national treasury.
Bool Function DepositGold(Int amount) Global Native

; Advance the simulation day (executing daily 24h economic, demographic, and diplomatic loops).
; Returns the new simulation day number.
Int Function AdvanceDay() Global Native
