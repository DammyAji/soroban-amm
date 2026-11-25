import { describe, expect, it } from "vitest";
import { StakingClient } from "./staking.js";

const staking = new StakingClient({
  rpcUrl: "https://soroban-testnet.stellar.org",
  networkPassphrase: "Test SDF Network ; September 2015",
  contractId: "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4",
});

describe("StakingClient.boostMultiplierHuman", () => {
  // Keep in sync with BOOST_SCALE in contracts/staking/src/lib.rs.
  // MIN_BOOST = 10_000 and DEFAULT_MAX_BOOST = 25_000.
  it.each([
    [10_000n, 1],
    [25_000n, 2.5],
    [40_000n, 4],
  ])("converts %s from BOOST_SCALE units to a human multiplier", (raw, expected) => {
    expect(staking.boostMultiplierHuman(raw)).toBe(expected);
  });
});
