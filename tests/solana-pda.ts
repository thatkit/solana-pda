import { expect } from 'chai';
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';

import { SolanaPda } from '../target/types/solana_pda';

const generateUniqueId = (): number => Math.round(Math.random() * 1e6);

const makeTxExplorerUrl = (txSignature: string): string =>
  `https://explorer.solana.com/tx/${txSignature}?cluster=custom`;

describe('PDA CRUD program', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.solanaPda as Program<SolanaPda>;

  const uniqueId = generateUniqueId();
  const seeds = <Array<Buffer | Uint8Array>>[
    Buffer.from('message'),
    Buffer.from(String(uniqueId)),
    provider.wallet.publicKey.toBuffer(),
  ];

  const [messagePda, messageBump] = PublicKey.findProgramAddressSync(
    seeds,
    program.programId,
  );

  console.log('Expected PDA:', {
    address: messagePda.toString(),
    bump: messageBump,
  });

  it('Should create Message Account', async () => {
    const message = 'Hello!!!';
    const transactionSignature = await program.methods
      .create(message, uniqueId)
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
      makeTxExplorerUrl(transactionSignature),
    );

    expect(messageAccount.bump).equal(messageBump);
  });

  it('Should update Message Account', async () => {
    const newMessage = 'Hello, Solana!';
    const transactionSignature = await program.methods
      .update(newMessage, uniqueId)
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
      makeTxExplorerUrl(transactionSignature),
    );

    expect(messageAccount.bump).equal(messageBump);
    expect(messageAccount.message).equal(newMessage);
  });

  it('Should delete Message Account', async () => {
    const transactionSignature = await program.methods
      .delete(uniqueId)
      .accounts({
        messageAccount: messagePda, // do we need this? (types are not generated correctly)
      } as any)
      .rpc({ commitment: 'confirmed' });

    const messageAccount = await program.account.messageAccount.fetchNullable(
      messagePda,
      'confirmed',
    );

    console.log(
      'Transaction Signature:',
      makeTxExplorerUrl(transactionSignature),
    );

    expect(messageAccount).equal(null);
  });
});
