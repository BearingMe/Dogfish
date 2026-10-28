// Minimal C++ Papyrus native function binding bridge for Dogfish.
//
// Milestone M92: Real Papyrus Native Function Registration.
// Uses pinned SKSE reference headers from references/sources/skse64/skse64/.

#include "skse64/PluginAPI.h"
#include "skse64/PapyrusVM.h"
#include "skse64/PapyrusNativeFunctions.h"

extern "C" {
    // Rust-exported functions with 32-bit Papyrus-compatible signatures
    SInt32 Dogfish_ClaimLocation_C(StaticFunctionTag* base, UInt32 locationID, bool hasStagingCamp);
    SInt32 Dogfish_GetHoldingOwner_C(StaticFunctionTag* base, UInt32 locationID);
    SInt32 Dogfish_GetTreasuryGold_C(StaticFunctionTag* base);
    bool   Dogfish_DepositGold_C(StaticFunctionTag* base, SInt32 amount);
    SInt32 Dogfish_AdvanceDay_C(StaticFunctionTag* base);
}

// Binds all declared DogfishNativeBridge functions to the Skyrim Papyrus VM.
extern "C" bool RegisterDogfishPapyrusBridge(VMClassRegistry* registry) {
    if (!registry) {
        return false;
    }

    // 1. DogfishNativeBridge.ClaimLocation(Int locationID, Bool hasStagingCamp) -> Int
    registry->RegisterFunction(
        new NativeFunction2<StaticFunctionTag, SInt32, UInt32, bool>(
            "ClaimLocation", "DogfishNativeBridge", Dogfish_ClaimLocation_C, registry));

    // 2. DogfishNativeBridge.GetHoldingOwner(Int locationID) -> Int
    registry->RegisterFunction(
        new NativeFunction1<StaticFunctionTag, SInt32, UInt32>(
            "GetHoldingOwner", "DogfishNativeBridge", Dogfish_GetHoldingOwner_C, registry));

    // 3. DogfishNativeBridge.GetTreasuryGold() -> Int
    registry->RegisterFunction(
        new NativeFunction0<StaticFunctionTag, SInt32>(
            "GetTreasuryGold", "DogfishNativeBridge", Dogfish_GetTreasuryGold_C, registry));

    // 4. DogfishNativeBridge.DepositGold(Int amount) -> Bool
    registry->RegisterFunction(
        new NativeFunction1<StaticFunctionTag, bool, SInt32>(
            "DepositGold", "DogfishNativeBridge", Dogfish_DepositGold_C, registry));

    // 5. DogfishNativeBridge.AdvanceDay() -> Int
    registry->RegisterFunction(
        new NativeFunction0<StaticFunctionTag, SInt32>(
            "AdvanceDay", "DogfishNativeBridge", Dogfish_AdvanceDay_C, registry));

    return true;
}
