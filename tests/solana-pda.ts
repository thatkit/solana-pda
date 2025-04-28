import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { SolanaPda } from '../target/types/solana_pda';

describe('solana-pda', () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solanaPda as Program<SolanaPda>;

  it('Should create', async () => {
    const tx = await program.methods.create().rpc();
    console.log('Tx signature:', tx);
  });

  // it('Should get without fee', async () => {

  // });

  it('Should update', async () => {
    const tx = await program.methods.update().rpc();
    console.log('Tx signature:', tx);
  });

  it('Should delete', async () => {
    const tx = await program.methods.delete().rpc();
    console.log('Tx signature:', tx);
  });
});
