import { AnchorProvider, Program, web3 } from '@project-serum/anchor';
import idl from '../target/idl/network.json';  // Ensure this path is correct

const { Keypair, PublicKey, SystemProgram } = web3;

// Setup the provider
const provider = AnchorProvider.env();  // This assumes you have environment variables configured for your wallet
const program = new Program(idl, new PublicKey('YourDeployedProgramId'), provider);

async function main() {
  const profile = Keypair.generate();

  // Interact with your Solana program
  await program.rpc.createProfile("John Doe", "Blockchain enthusiast", {
    accounts: {
      profile: profile.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [profile],
  });

  // Fetch and log the created profile
  const profileAccount = await program.account.profile.fetch(profile.publicKey);
  console.log(`Name: ${profileAccount.name}, Bio: ${profileAccount.bio}`);
}

main().catch(err => console.error(err));
