import { RedspotUserConfig } from 'redspot/types'
import '@redspot/patract'
import '@redspot/chai'
import '@redspot/gas-reporter'

export default {
    defaultNetwork: 'development',
    contract: {
        ink: {
            toolchain: 'nightly-2022-08-15',
            sources: ['contracts/rmrk/**']
        }
    },
    networks: {
        development: {
            endpoint: 'ws://127.0.0.1:9944',
            gasLimit: '400000000000',
            explorerUrl: 'https://polkadot.js.org/apps/#/explorer/query/?rpc=ws://127.0.0.1:9944/',
            types: {
                PSP22Error: {
                    _enum: {
                        Custom: 'String',
                        InsufficientBalance: null,
                        InsufficientAllowance: null,
                        ZeroRecipientAddress: null,
                        ZeroSenderAddress: null,
                        SafeTransferCheckFailed: 'String',
                    }
                },
                OwnableError: {
                    _enum: {
                        CallerIsNotOwner: null,
                        NewOwnerIsZero: null,
                    }
                },
                PausableError: {
                    _enum: {
                        Paused: null,
                        NotPaused: null,
                    }
                },
                OpenbrushContractsErrorsPsp22Psp22Error: {
                    _enum: {
                        Custom: 'String',
                        InsufficientBalance: null,
                        InsufficientAllowance: null,
                        ZeroRecipientAddress: null,
                        ZeroSenderAddress: null,
                        SafeTransferCheckFailed: 'String',
                    }
                }
            },
        },
        substrate: {
            endpoint: 'ws://127.0.0.1:9944',
            gasLimit: '400000000000',
            accounts: ['//Alice'],
        }
    },
    mocha: {
        timeout: 600000
    }
} as RedspotUserConfig
