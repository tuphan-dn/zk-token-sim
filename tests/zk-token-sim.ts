import {
  Program,
  Provider,
  setProvider,
  web3,
  workspace,
} from '@project-serum/anchor'
import { ZkTokenSim } from '../target/types/zk_token_sim'
import { Point } from '../lib/point'

describe('zk-token-sim', () => {
  setProvider(Provider.env())
  let program: Program<ZkTokenSim>, mint: web3.Keypair, account: web3.PublicKey

  before(async () => {
    program = workspace.ZkTokenSim
    mint = web3.Keypair.generate()
    const [pubkey] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from('account'),
        mint.publicKey.toBuffer(),
        program.provider.wallet.publicKey.toBuffer(),
      ],
      program.programId,
    )
    account = pubkey
  })

  it('initialize mint', async () => {
    const point = Point.INFINITY.toBuffer()
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
    const data = await program.account.mint.fetch(mint.publicKey)
    console.log(Point.INFINITY.toBuffer())
    console.log(Point.fromPublicKey(data.supplyCommitment).toBuffer())
  })

  it('initialize account', async () => {
    await program.rpc.initializeAccount({
      accounts: {
        mint: mint.publicKey,
        account,
        authority: program.provider.wallet.publicKey,
        owner: program.provider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
      },
    })
  })

  it('fetch account', async () => {
    const data = await program.account.account.fetch(account)
    console.log(Point.fromPublicKey(data.amountCommitment).toBuffer())
  })
})
