import { expect } from 'chai';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';

import { SolanaPda } from '../target/types/solana_pda';

const generateUniqueSeedData = (): string => 'TEST' + String(Date.now());

describe('solana-pda', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solanaPda as Program<SolanaPda>;

  const uniqueSeedData = generateUniqueSeedData();
  const seeds = <Array<Buffer | Uint8Array>>[
    Buffer.from('message'),
    Buffer.from(uniqueSeedData),
    provider.wallet.publicKey.toBuffer(),
  ];

  const [messagePda, messageBump] = PublicKey.findProgramAddressSync(
    seeds,
    program.programId
  );

  console.log('Expected PDA:', {
    address: messagePda.toString(),
    bump: messageBump,
  });

  it('Create Message Account', async () => {
    const message = uniqueSeedData;
    const transactionSignature = await program.methods
      .create(message)
      .accounts({
        messageAccount: messagePda, // do we need this? (types are not generated correctly)
      } as any)
      .rpc({ commitment: 'confirmed' });

    const messageAccount = await program.account.messageAccount.fetch(
      messagePda,
      'confirmed',
    );

    console.log(JSON.stringify(messageAccount, null, 2));
    console.log(
      'Transaction Signature:',
      `https://explorer.solana.com/tx/${transactionSignature}?cluster=custom`,
    );

    expect(messageAccount.bump).equal(messageBump);
  });
});
