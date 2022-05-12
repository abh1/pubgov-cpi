import * as anchor from '@project-serum/anchor';
import { Drafting } from '../target/types/drafting';
import { Connection, PublicKey } from "@solana/web3.js";
import { Program, Provider, web3 } from "@project-serum/anchor";
const assert = require("assert");
const { SystemProgram } = anchor.web3;
const splToken = require("@solana/spl-token");



export const NODE_RPC = "https://api.devnet.solana.com"; // devnet environment
export const CONNECTION = new Connection(NODE_RPC);

console.log("account: 111");

describe('drafting', () => {

  const provider = anchor.Provider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const idl = JSON.parse(require('fs').readFileSync('/Users/abhinavsharma/sn/SapienPayout/draftreporting/drafting/target/idl/newsreport.json', 'utf8'));
  const programId = new anchor.web3.PublicKey('FuiSWC8pz48qFicr9FyhEDMaMot9iNLQHZmjq66tcvUp');

  console.log(programId);

  console.log("account: 222");
  // Generate the program client from IDL.
  const prog = new anchor.Program(idl, programId);

  const mint = new PublicKey("3qq7ExpwRRAAexGNpUVoFkiTfSB1uo8ezsbyAoxhyryo");

  const RepAccount = anchor.web3.Keypair.generate();
  const myAccount = anchor.web3.Keypair.generate();
  const Acc2 = provider.wallet.publicKey;

  console.log("account: 333");

  console.log("ma", myAccount.publicKey);
  console.log("providerAccount :", provider.wallet.publicKey);
  console.log("Account :", SystemProgram.programId);


  it('Is initialized!', async () => {
    const fromTokenAccount = await splToken.getOrCreateAssociatedTokenAccount(
      CONNECTION,
      myAccount,
      mint,
      myAccount.publicKey
    );

    const treasuryTokenAccount = await splToken.getOrCreateAssociatedTokenAccount(
      CONNECTION,
      myAccount,
      mint,
      new PublicKey("6kgSK2hFDjUCS3wafYYW2VSwkjETuqHdByWddwmytyp7")
    );

      const tx = await prog.rpc.initialize({
        accounts: {
          reportAccount: RepAccount.publicKey,
          authority: myAccount.publicKey,
          newstoken: splToken.TOKEN_PROGRAM_ID,
          from: fromTokenAccount.address,
          treasury: treasuryTokenAccount.address,
          systemProgram: SystemProgram.programId,
        },
        signers: [RepAccount],
      });
  
      const account = await prog.account.reportAccount.fetch(
        RepAccount.publicKey
      );

      console.log("Your transaction signature", tx);

      console.log("account: ", account);

      console.log("fromTokenAccount: ", fromTokenAccount);

      console.log("treasuryTokenAccount: ", treasuryTokenAccount);
  
      const tx2 = await prog.rpc.updateReport("Avadakedavra", {
        accounts: {
          reportAccount: RepAccount.publicKey,
          authority: myAccount.publicKey,
          newstoken: splToken.TOKEN_PROGRAM_ID,
          from: fromTokenAccount.address,
          treasury: treasuryTokenAccount.address,
          systemProgram: SystemProgram.programId,
        },
        signers: [myAccount],
      }); 


    console.log("Your transaction2222 signature", tx2);
    
      const tx3 = await prog.rpc.pushForVote("Avadakedavra", 1000000000, {
        accounts: {
          reportAccount: RepAccount.publicKey,
          authority: myAccount.publicKey,
          newstoken: splToken.TOKEN_PROGRAM_ID,
          from: fromTokenAccount.address,
          treasury: treasuryTokenAccount.address,
          systemProgram: SystemProgram.programId,
        },
        signers: [myAccount],
      }); 


    console.log("Your transaction333333 signature", tx3);

  });

});

