import {
  Program,
  Provider,
  setProvider,
  web3,
  workspace,
} from '@project-serum/anchor'
import { ZkTokenSim } from '../target/types/zk_token_sim'

describe('zk-token-sim', () => {
  setProvider(Provider.env())
  const mint = web3.Keypair.generate()
  const program = workspace.ZkTokenSim as Program<ZkTokenSim>

  it('initialize mint', async () => {
    const point = Buffer.from([
      0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
      0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
      0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
    ])
    const pubkey: web3.PublicKey = web3.PublicKey.decode(point)
    await program.rpc.initializeMint(pubkey, pubkey, {
      accounts: {
        mint: mint.publicKey,
        authority: program.provider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [mint],
    })
  })

  it('fetch mint', async () => {
    console.log(program.provider.wallet.publicKey.toBase58())
    const data = await program.account.mint.fetch(mint.publicKey)
    console.log(data.authority.toBase58())
  })
})
