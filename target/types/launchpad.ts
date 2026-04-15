/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/launchpad.json`.
 */
export type Launchpad = {
  "address": "6Hdc2yw7PRFaxgw39MkHfPH31gjt1wgZHvYoxDsp6yvv",
  "metadata": {
    "name": "launchpad",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Solana token launchpad with bonding curve and presale modes"
  },
  "instructions": [
    {
      "name": "acceptAdmin",
      "discriminator": [
        112,
        42,
        45,
        90,
        116,
        181,
        13,
        170
      ],
      "accounts": [
        {
          "name": "newAdmin",
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "buyBonding",
      "discriminator": [
        223,
        204,
        211,
        74,
        11,
        184,
        179,
        111
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "buyerPosition",
          "docs": [
            "Per-wallet position tracking for max buy enforcement (H-2 fix)"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  115,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "tokenVault",
          "docs": [
            "Token vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "buyerTokenAccount",
          "docs": [
            "Buyer's token account"
          ],
          "writable": true
        },
        {
          "name": "devWallet",
          "writable": true
        },
        {
          "name": "platformWallet",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "solAmount",
          "type": "u64"
        },
        {
          "name": "minTokensOut",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimPresale",
      "discriminator": [
        82,
        240,
        122,
        5,
        109,
        66,
        86,
        190
      ],
      "accounts": [
        {
          "name": "user",
          "signer": true
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "userPosition",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  115,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "tokenVault",
          "docs": [
            "Token vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "docs": [
            "User's token account"
          ],
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "contributePresale",
      "discriminator": [
        248,
        72,
        28,
        96,
        70,
        166,
        8,
        117
      ],
      "accounts": [
        {
          "name": "contributor",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "userPosition",
          "docs": [
            "User's position for this pool (init-if-needed for first contribution)"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  115,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              },
              {
                "kind": "account",
                "path": "contributor"
              }
            ]
          }
        },
        {
          "name": "platformWallet",
          "docs": [
            "Platform wallet for presale fee"
          ],
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "solAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "createBondingPool",
      "discriminator": [
        240,
        13,
        25,
        154,
        169,
        153,
        25,
        43
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "mint",
          "docs": [
            "Token mint — creator must be mint authority initially"
          ],
          "writable": true
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault PDA holding native SOL lamports."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "tokenVault",
          "docs": [
            "Token vault holding the token supply"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "createBondingPoolParams"
            }
          }
        }
      ]
    },
    {
      "name": "createPresalePool",
      "discriminator": [
        255,
        111,
        192,
        70,
        143,
        255,
        232,
        85
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "mint",
          "docs": [
            "Token mint — creator must be mint authority"
          ],
          "writable": true
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "tokenVault",
          "docs": [
            "Token vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "createPresalePoolParams"
            }
          }
        }
      ]
    },
    {
      "name": "executeBuyback",
      "discriminator": [
        47,
        32,
        19,
        100,
        184,
        96,
        144,
        49
      ],
      "accounts": [
        {
          "name": "payer",
          "docs": [
            "Anyone can trigger buyback (permissionless crank)"
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "buybackState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  121,
                  98,
                  97,
                  99,
                  107
                ]
              },
              {
                "kind": "account",
                "path": "buyback_state.pool",
                "account": "buybackState"
              }
            ]
          }
        },
        {
          "name": "buybackSolVault",
          "docs": [
            "SOL vault PDA that holds the buyback treasury SOL.",
            "This is the pool's sol_vault — validated via seeds in the handler",
            "because seeds depend on pool_type (bonding vs presale)."
          ],
          "writable": true
        },
        {
          "name": "poolMint"
        },
        {
          "name": "payerWsolAccount",
          "docs": [
            "Payer's WSOL account (SOL gets wrapped here for the swap)"
          ],
          "writable": true
        },
        {
          "name": "payerTokenAccount",
          "docs": [
            "Payer's token account to receive bought-back tokens"
          ],
          "writable": true
        },
        {
          "name": "tokenMint",
          "docs": [
            "Token mint (for burning)"
          ],
          "writable": true
        },
        {
          "name": "meteoraProgram"
        },
        {
          "name": "meteoraPool",
          "writable": true
        },
        {
          "name": "meteoraInputVault",
          "writable": true
        },
        {
          "name": "meteoraOutputVault",
          "writable": true
        },
        {
          "name": "wsolMint"
        },
        {
          "name": "protocolFee",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "executeBuybackParams"
            }
          }
        }
      ]
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "initializeParams"
            }
          }
        }
      ]
    },
    {
      "name": "migrateBonding",
      "discriminator": [
        171,
        184,
        81,
        127,
        253,
        22,
        158,
        206
      ],
      "accounts": [
        {
          "name": "payer",
          "docs": [
            "C-6: Only admin can trigger migration"
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "tokenVault",
          "docs": [
            "Token vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "buybackState",
          "docs": [
            "Buyback state account (init)"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  121,
                  98,
                  97,
                  99,
                  107
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              }
            ]
          }
        },
        {
          "name": "platformWallet",
          "docs": [
            "Platform wallet receives migration fee"
          ],
          "writable": true
        },
        {
          "name": "meteoraProgram"
        },
        {
          "name": "meteoraPool",
          "writable": true
        },
        {
          "name": "meteoraPoolConfig"
        },
        {
          "name": "positionNftMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "positionNftAccount",
          "writable": true
        },
        {
          "name": "positionAccount",
          "writable": true
        },
        {
          "name": "positionNftMetadata",
          "writable": true
        },
        {
          "name": "meteoraVaultA",
          "writable": true
        },
        {
          "name": "meteoraVaultB",
          "writable": true
        },
        {
          "name": "wsolMint",
          "docs": [
            "C-7: WSOL mint — validated to be the canonical native mint"
          ]
        },
        {
          "name": "tokenMint",
          "docs": [
            "H-4: Actual token mint (for Meteora pool creation)"
          ]
        },
        {
          "name": "payerWsolAccount",
          "writable": true
        },
        {
          "name": "payerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "migratePresale",
      "discriminator": [
        10,
        76,
        8,
        246,
        151,
        28,
        91,
        25
      ],
      "accounts": [
        {
          "name": "payer",
          "docs": [
            "C-6: Only admin can trigger migration"
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "tokenVault",
          "docs": [
            "Token vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "buybackState",
          "docs": [
            "Buyback state account"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  121,
                  98,
                  97,
                  99,
                  107
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              }
            ]
          }
        },
        {
          "name": "platformWallet",
          "docs": [
            "Platform wallet"
          ],
          "writable": true
        },
        {
          "name": "creatorWallet",
          "docs": [
            "Creator wallet for creator pool allocation"
          ],
          "writable": true
        },
        {
          "name": "meteoraProgram"
        },
        {
          "name": "meteoraPool",
          "writable": true
        },
        {
          "name": "meteoraPoolConfig"
        },
        {
          "name": "positionNftMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "positionNftAccount",
          "writable": true
        },
        {
          "name": "positionAccount",
          "writable": true
        },
        {
          "name": "positionNftMetadata",
          "writable": true
        },
        {
          "name": "meteoraVaultA",
          "writable": true
        },
        {
          "name": "meteoraVaultB",
          "writable": true
        },
        {
          "name": "wsolMint",
          "docs": [
            "C-7: WSOL mint validated"
          ]
        },
        {
          "name": "tokenMint",
          "docs": [
            "H-4: Token mint for Meteora"
          ]
        },
        {
          "name": "payerWsolAccount",
          "writable": true
        },
        {
          "name": "payerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "pause",
      "discriminator": [
        211,
        22,
        221,
        251,
        74,
        121,
        193,
        47
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "proposeAdmin",
      "discriminator": [
        121,
        214,
        199,
        212,
        87,
        39,
        117,
        234
      ],
      "accounts": [
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "newAdmin",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "refundPresale",
      "discriminator": [
        70,
        192,
        242,
        198,
        106,
        202,
        142,
        48
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "userPosition",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  115,
                  105,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "pool"
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  95,
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "presalePool"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "sellBonding",
      "discriminator": [
        160,
        199,
        9,
        23,
        236,
        33,
        49,
        0
      ],
      "accounts": [
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "solVault",
          "docs": [
            "SOL vault PDA"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  115,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "tokenVault",
          "docs": [
            "Token vault"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  95,
                  116,
                  111,
                  107,
                  101,
                  110,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "pool.mint",
                "account": "bondingCurvePool"
              }
            ]
          }
        },
        {
          "name": "sellerTokenAccount",
          "docs": [
            "Seller's token account"
          ],
          "writable": true
        },
        {
          "name": "platformWallet",
          "docs": [
            "Platform wallet receives platform fee"
          ],
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "tokenAmount",
          "type": "u64"
        },
        {
          "name": "minSolOut",
          "type": "u64"
        }
      ]
    },
    {
      "name": "unpause",
      "discriminator": [
        169,
        144,
        4,
        38,
        10,
        141,
        188,
        255
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "updateConfig",
      "discriminator": [
        29,
        158,
        252,
        191,
        10,
        83,
        219,
        99
      ],
      "accounts": [
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "updateConfigParams"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "bondingCurvePool",
      "discriminator": [
        167,
        155,
        150,
        227,
        65,
        117,
        3,
        86
      ]
    },
    {
      "name": "buybackState",
      "discriminator": [
        247,
        168,
        248,
        43,
        167,
        32,
        210,
        84
      ]
    },
    {
      "name": "globalConfig",
      "discriminator": [
        149,
        8,
        156,
        202,
        160,
        252,
        176,
        217
      ]
    },
    {
      "name": "presalePool",
      "discriminator": [
        40,
        45,
        173,
        158,
        138,
        60,
        76,
        118
      ]
    },
    {
      "name": "userPosition",
      "discriminator": [
        251,
        248,
        209,
        245,
        83,
        234,
        17,
        27
      ]
    }
  ],
  "events": [
    {
      "name": "buybackExecuted",
      "discriminator": [
        150,
        109,
        157,
        10,
        124,
        24,
        38,
        189
      ]
    },
    {
      "name": "configUpdated",
      "discriminator": [
        40,
        241,
        230,
        122,
        11,
        19,
        198,
        194
      ]
    },
    {
      "name": "migrationCompleted",
      "discriminator": [
        223,
        45,
        123,
        192,
        106,
        249,
        6,
        241
      ]
    },
    {
      "name": "migrationReady",
      "discriminator": [
        15,
        71,
        184,
        9,
        194,
        36,
        174,
        235
      ]
    },
    {
      "name": "poolCreated",
      "discriminator": [
        202,
        44,
        41,
        88,
        104,
        220,
        157,
        82
      ]
    },
    {
      "name": "presaleClaimed",
      "discriminator": [
        71,
        206,
        74,
        134,
        208,
        95,
        73,
        99
      ]
    },
    {
      "name": "presaleContribution",
      "discriminator": [
        236,
        32,
        200,
        82,
        145,
        235,
        141,
        28
      ]
    },
    {
      "name": "presaleRefunded",
      "discriminator": [
        136,
        243,
        241,
        106,
        87,
        219,
        231,
        232
      ]
    },
    {
      "name": "tokensBought",
      "discriminator": [
        151,
        148,
        173,
        226,
        128,
        30,
        249,
        190
      ]
    },
    {
      "name": "tokensSold",
      "discriminator": [
        217,
        83,
        68,
        137,
        134,
        225,
        94,
        45
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "mathOverflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6001,
      "name": "mathUnderflow",
      "msg": "Arithmetic underflow"
    },
    {
      "code": 6002,
      "name": "divisionByZero",
      "msg": "Division by zero"
    },
    {
      "code": 6003,
      "name": "castOverflow",
      "msg": "Result exceeds u64 range"
    },
    {
      "code": 6004,
      "name": "unauthorizedAdmin",
      "msg": "Unauthorized: not admin"
    },
    {
      "code": 6005,
      "name": "unauthorizedPauseAuthority",
      "msg": "Unauthorized: not pause authority"
    },
    {
      "code": 6006,
      "name": "unauthorizedCreator",
      "msg": "Unauthorized: not pool creator"
    },
    {
      "code": 6007,
      "name": "platformPaused",
      "msg": "Platform is paused"
    },
    {
      "code": 6008,
      "name": "poolPaused",
      "msg": "Pool is paused"
    },
    {
      "code": 6009,
      "name": "alreadyMigrated",
      "msg": "Pool already migrated"
    },
    {
      "code": 6010,
      "name": "notMigrated",
      "msg": "Pool not migrated yet"
    },
    {
      "code": 6011,
      "name": "migrationTargetNotReached",
      "msg": "Migration target not reached"
    },
    {
      "code": 6012,
      "name": "poolNotActive",
      "msg": "Pool is not active"
    },
    {
      "code": 6013,
      "name": "exceedsMaxBuy",
      "msg": "Buy amount exceeds max 1% per wallet"
    },
    {
      "code": 6014,
      "name": "insufficientTokenReserves",
      "msg": "Insufficient token reserves"
    },
    {
      "code": 6015,
      "name": "insufficientSolReserves",
      "msg": "Insufficient SOL reserves"
    },
    {
      "code": 6016,
      "name": "slippageExceeded",
      "msg": "Slippage tolerance exceeded"
    },
    {
      "code": 6017,
      "name": "zeroAmount",
      "msg": "Amount must be greater than zero"
    },
    {
      "code": 6018,
      "name": "presaleEnded",
      "msg": "Presale has ended"
    },
    {
      "code": 6019,
      "name": "presaleNotEnded",
      "msg": "Presale has not ended yet"
    },
    {
      "code": 6020,
      "name": "exceedsMaxContribution",
      "msg": "Contribution exceeds max 1% per wallet"
    },
    {
      "code": 6021,
      "name": "alreadyClaimed",
      "msg": "Tokens already claimed"
    },
    {
      "code": 6022,
      "name": "alreadyRefunded",
      "msg": "Refund already claimed"
    },
    {
      "code": 6023,
      "name": "targetReached",
      "msg": "Presale target was reached, no refund"
    },
    {
      "code": 6024,
      "name": "invalidMigrationTarget",
      "msg": "Invalid migration target: must be 100-10000 SOL"
    },
    {
      "code": 6025,
      "name": "invalidEndTime",
      "msg": "Invalid end time: must be in the future"
    },
    {
      "code": 6026,
      "name": "buybackTooFrequent",
      "msg": "Buyback rate limit: too frequent"
    },
    {
      "code": 6027,
      "name": "insufficientTreasury",
      "msg": "Insufficient treasury balance for buyback"
    },
    {
      "code": 6028,
      "name": "invalidBuybackMode",
      "msg": "Invalid buyback mode"
    },
    {
      "code": 6029,
      "name": "invalidFeeConfig",
      "msg": "Invalid fee configuration"
    },
    {
      "code": 6030,
      "name": "invalidPoolParams",
      "msg": "Invalid pool parameters"
    }
  ],
  "types": [
    {
      "name": "bondingCurvePool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "docs": [
              "Pool creator"
            ],
            "type": "pubkey"
          },
          {
            "name": "mint",
            "docs": [
              "Token mint"
            ],
            "type": "pubkey"
          },
          {
            "name": "virtualSolReserves",
            "docs": [
              "Virtual SOL reserves for price calculation"
            ],
            "type": "u64"
          },
          {
            "name": "virtualTokenReserves",
            "docs": [
              "Virtual token reserves for price calculation"
            ],
            "type": "u64"
          },
          {
            "name": "realSolReserves",
            "docs": [
              "Actual SOL collected (in lamports)"
            ],
            "type": "u64"
          },
          {
            "name": "realTokenReserves",
            "docs": [
              "Actual tokens remaining in vault"
            ],
            "type": "u64"
          },
          {
            "name": "initialRealTokenReserves",
            "docs": [
              "Initial token supply loaded into vault"
            ],
            "type": "u64"
          },
          {
            "name": "migrationTarget",
            "docs": [
              "SOL target for migration (lamports, default: 100 SOL)"
            ],
            "type": "u64"
          },
          {
            "name": "maxBuyBps",
            "docs": [
              "Max buy percentage in basis points (100 = 1%)"
            ],
            "type": "u16"
          },
          {
            "name": "buybackTreasury",
            "docs": [
              "Accumulated sell tax SOL for buyback treasury (lamports)"
            ],
            "type": "u64"
          },
          {
            "name": "isMigrated",
            "docs": [
              "Pool has been migrated to Meteora"
            ],
            "type": "bool"
          },
          {
            "name": "isPaused",
            "docs": [
              "Pool-level pause"
            ],
            "type": "bool"
          },
          {
            "name": "bump",
            "docs": [
              "PDA bump"
            ],
            "type": "u8"
          },
          {
            "name": "solVaultBump",
            "docs": [
              "SOL vault bump"
            ],
            "type": "u8"
          },
          {
            "name": "tokenVaultBump",
            "docs": [
              "Token vault bump"
            ],
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "buybackExecuted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "solSpent",
            "type": "u64"
          },
          {
            "name": "tokensReceived",
            "type": "u64"
          },
          {
            "name": "mode",
            "type": "u8"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "buybackMode",
      "docs": [
        "Buyback modes for presale"
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "burn"
          },
          {
            "name": "addLiquidity"
          }
        ]
      }
    },
    {
      "name": "buybackState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "docs": [
              "Associated pool pubkey"
            ],
            "type": "pubkey"
          },
          {
            "name": "mint",
            "docs": [
              "Associated token mint"
            ],
            "type": "pubkey"
          },
          {
            "name": "treasuryBalance",
            "docs": [
              "SOL remaining in buyback treasury (lamports)"
            ],
            "type": "u64"
          },
          {
            "name": "lastBuybackSlot",
            "docs": [
              "Last slot a buyback was executed"
            ],
            "type": "u64"
          },
          {
            "name": "totalSolSpent",
            "docs": [
              "Total SOL spent on buybacks"
            ],
            "type": "u64"
          },
          {
            "name": "totalTokensBought",
            "docs": [
              "Total tokens bought back"
            ],
            "type": "u64"
          },
          {
            "name": "totalTokensBurned",
            "docs": [
              "Total tokens burned"
            ],
            "type": "u64"
          },
          {
            "name": "totalTokensLp",
            "docs": [
              "Total tokens added as liquidity"
            ],
            "type": "u64"
          },
          {
            "name": "poolType",
            "docs": [
              "Pool type (0 = bonding, 1 = presale)"
            ],
            "type": "u8"
          },
          {
            "name": "bump",
            "docs": [
              "PDA bump"
            ],
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "configUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "createBondingPoolParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "virtualSolReserves",
            "docs": [
              "Token name (for mint metadata, not stored on-chain in this program)"
            ],
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "virtualTokenReserves",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "tokenSupply",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "migrationTarget",
            "type": {
              "option": "u64"
            }
          }
        ]
      }
    },
    {
      "name": "createPresalePoolParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "migrationTarget",
            "docs": [
              "Migration target in lamports (100-10,000 SOL)"
            ],
            "type": "u64"
          },
          {
            "name": "tokenSupply",
            "docs": [
              "Total token supply to distribute"
            ],
            "type": "u64"
          },
          {
            "name": "endTime",
            "docs": [
              "Presale end time (unix timestamp)"
            ],
            "type": "i64"
          },
          {
            "name": "creatorPoolBps",
            "docs": [
              "Creator pool percentage in bps (default 2000 = 20%)"
            ],
            "type": {
              "option": "u16"
            }
          }
        ]
      }
    },
    {
      "name": "executeBuybackParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mode",
            "docs": [
              "Buyback mode: Burn or AddLiquidity"
            ],
            "type": {
              "defined": {
                "name": "buybackMode"
              }
            }
          },
          {
            "name": "minTokensOut",
            "docs": [
              "Minimum tokens expected (slippage protection)"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "globalConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "docs": [
              "Full admin authority"
            ],
            "type": "pubkey"
          },
          {
            "name": "pauseAuthority",
            "docs": [
              "Can pause/unpause only"
            ],
            "type": "pubkey"
          },
          {
            "name": "devWallet",
            "docs": [
              "Receives dev portion of fees"
            ],
            "type": "pubkey"
          },
          {
            "name": "platformWallet",
            "docs": [
              "Receives platform portion of fees"
            ],
            "type": "pubkey"
          },
          {
            "name": "devFeeBps",
            "docs": [
              "Dev fee in basis points (default: 50 = 0.5%)"
            ],
            "type": "u16"
          },
          {
            "name": "platformFeeBps",
            "docs": [
              "Platform fee in basis points (default: 50 = 0.5%)"
            ],
            "type": "u16"
          },
          {
            "name": "sellTaxBps",
            "docs": [
              "Sell tax in basis points (default: 2400 = 24%)"
            ],
            "type": "u16"
          },
          {
            "name": "presalePlatformFeeBps",
            "docs": [
              "Presale buy fee in basis points (default: 100 = 1%)"
            ],
            "type": "u16"
          },
          {
            "name": "migrationFeeBps",
            "docs": [
              "Migration fee in basis points (default: 100 = 1%)"
            ],
            "type": "u16"
          },
          {
            "name": "pendingAdmin",
            "docs": [
              "Pending admin for two-step transfer (Pubkey::default() = none)"
            ],
            "type": "pubkey"
          },
          {
            "name": "isPaused",
            "docs": [
              "Global pause flag"
            ],
            "type": "bool"
          },
          {
            "name": "bump",
            "docs": [
              "PDA bump"
            ],
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "initializeParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pauseAuthority",
            "type": "pubkey"
          },
          {
            "name": "devWallet",
            "type": "pubkey"
          },
          {
            "name": "platformWallet",
            "type": "pubkey"
          },
          {
            "name": "devFeeBps",
            "type": "u16"
          },
          {
            "name": "platformFeeBps",
            "type": "u16"
          },
          {
            "name": "sellTaxBps",
            "type": "u16"
          },
          {
            "name": "presalePlatformFeeBps",
            "type": "u16"
          },
          {
            "name": "migrationFeeBps",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "migrationCompleted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "poolType",
            "type": "u8"
          },
          {
            "name": "meteoraPool",
            "type": "pubkey"
          },
          {
            "name": "liquiditySol",
            "type": "u64"
          },
          {
            "name": "liquidityTokens",
            "type": "u64"
          },
          {
            "name": "platformFee",
            "type": "u64"
          },
          {
            "name": "buybackAllocation",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "migrationReady",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "solRaised",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "poolCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "poolType",
            "type": "u8"
          },
          {
            "name": "migrationTarget",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "presaleClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "presaleContribution",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "contributor",
            "type": "pubkey"
          },
          {
            "name": "solAmount",
            "type": "u64"
          },
          {
            "name": "totalRaised",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "presalePool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "docs": [
              "Pool creator"
            ],
            "type": "pubkey"
          },
          {
            "name": "mint",
            "docs": [
              "Token mint"
            ],
            "type": "pubkey"
          },
          {
            "name": "migrationTarget",
            "docs": [
              "SOL target for migration (lamports, 100-10000 SOL)"
            ],
            "type": "u64"
          },
          {
            "name": "currentRaised",
            "docs": [
              "Total SOL raised so far (lamports)"
            ],
            "type": "u64"
          },
          {
            "name": "totalTokenSupply",
            "docs": [
              "Total token supply for distribution"
            ],
            "type": "u64"
          },
          {
            "name": "maxBuyBps",
            "docs": [
              "Max contribution per wallet in basis points (100 = 1%)"
            ],
            "type": "u16"
          },
          {
            "name": "creatorPoolBps",
            "docs": [
              "Creator pool percentage in basis points (2000 = 20%)"
            ],
            "type": "u16"
          },
          {
            "name": "endTime",
            "docs": [
              "Presale end time (unix timestamp)"
            ],
            "type": "i64"
          },
          {
            "name": "numContributors",
            "docs": [
              "Number of unique contributors"
            ],
            "type": "u32"
          },
          {
            "name": "isMigrated",
            "docs": [
              "Pool has been migrated to Meteora"
            ],
            "type": "bool"
          },
          {
            "name": "bump",
            "docs": [
              "PDA bump"
            ],
            "type": "u8"
          },
          {
            "name": "solVaultBump",
            "docs": [
              "SOL vault bump"
            ],
            "type": "u8"
          },
          {
            "name": "tokenVaultBump",
            "docs": [
              "Token vault bump"
            ],
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "presaleRefunded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "solAmount",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tokensBought",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "solAmount",
            "type": "u64"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "devFee",
            "type": "u64"
          },
          {
            "name": "platformFee",
            "type": "u64"
          },
          {
            "name": "newPrice",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "tokensSold",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "pubkey"
          },
          {
            "name": "seller",
            "type": "pubkey"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          },
          {
            "name": "solAmount",
            "type": "u64"
          },
          {
            "name": "platformFee",
            "type": "u64"
          },
          {
            "name": "sellTax",
            "type": "u64"
          },
          {
            "name": "newPrice",
            "type": "u64"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "updateConfigParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "newPauseAuthority",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "newDevWallet",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "newPlatformWallet",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "newDevFeeBps",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "newPlatformFeeBps",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "newSellTaxBps",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "newPresalePlatformFeeBps",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "newMigrationFeeBps",
            "type": {
              "option": "u16"
            }
          }
        ]
      }
    },
    {
      "name": "userPosition",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "docs": [
              "User public key"
            ],
            "type": "pubkey"
          },
          {
            "name": "pool",
            "docs": [
              "Pool this position belongs to"
            ],
            "type": "pubkey"
          },
          {
            "name": "solContributed",
            "docs": [
              "Total SOL contributed (lamports)"
            ],
            "type": "u64"
          },
          {
            "name": "tokensClaimed",
            "docs": [
              "Whether tokens have been claimed (presale only)"
            ],
            "type": "bool"
          },
          {
            "name": "refundClaimed",
            "docs": [
              "Whether refund has been claimed (presale only)"
            ],
            "type": "bool"
          },
          {
            "name": "bump",
            "docs": [
              "PDA bump"
            ],
            "type": "u8"
          }
        ]
      }
    }
  ]
};
