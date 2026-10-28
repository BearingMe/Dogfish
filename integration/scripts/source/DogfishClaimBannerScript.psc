Scriptname DogfishClaimBannerScript extends ObjectReference
{Attached to the interactive claim banner activator inside DogfishTestCell.}

; The Holding ID associated with this claim point (0 = Whiterun Capital).
Int Property LocationID = 0 Auto

; Event fired when the player activates the banner activator in-world.
Event OnActivate(ObjectReference akActionRef)
    if akActionRef == Game.GetPlayer()
        int result = DogfishNativeBridge.ClaimLocation(LocationID, true)
        if result == 0
            Debug.Notification("Dogfish: Claimed Holding " + LocationID + " successfully!")
            Debug.Trace("[Dogfish] Holding " + LocationID + " claimed by player.")
        elseif result == 3
            Debug.Notification("Dogfish: Holding " + LocationID + " is already claimed by your faction.")
        elseif result == 2
            Debug.Notification("Dogfish: Holding " + LocationID + " cannot be claimed: not adjacent.")
        else
            Debug.Notification("Dogfish: Claim failed. Error code: " + result)
            Debug.Trace("[Dogfish] ClaimLocation failed with error code: " + result)
        endif
    endif
EndEvent
