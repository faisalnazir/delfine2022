import { writable } from 'svelte/store';
import { Connection, PublicKey } from '@solana/web3.js';
import { AnchorProvider, Program } from '@project-serum/anchor';

// Import the IDL (will be generated after building the program)
// import idl from '../../../target/idl/delfine.json';

// Placeholder IDL until the program is built
const IDL = {
	version: "0.1.0",
	name: "delfine",
	instructions: [],
	accounts: [],
	errors: []
};

const PROGRAM_ID = new PublicKey('De1FiNexxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx');
const NETWORK = 'devnet';
const opts = {
	preflightCommitment: 'processed'
};

function createWorkSpace() {
	const { subscribe, set, update } = writable(null);

	return {
		subscribe,

		/**
		 * Initialize the workspace with wallet and connection
		 * @param {Object} wallet - Solana wallet adapter wallet
		 */
		init: async (wallet) => {
			if (!wallet) {
				set(null);
				return;
			}

			try {
				// Create connection
				const connection = new Connection(
					`https://api.${NETWORK}.solana.com`,
					opts.preflightCommitment
				);

				// Create provider
				const provider = new AnchorProvider(
					connection,
					wallet,
					opts
				);

				// Create program interface
				const program = new Program(IDL, PROGRAM_ID, provider);

				// Update store
				set({
					wallet,
					connection,
					provider,
					program,
					network: NETWORK,
				});

				console.log('Solana workspace initialized', {
					wallet: wallet.publicKey?.toString(),
					network: NETWORK,
				});

			} catch (error) {
				console.error('Error initializing workspace:', error);
				set(null);
			}
		},

		/**
		 * Clear the workspace
		 */
		disconnect: () => {
			set(null);
		}
	};
}

export const workSpace = createWorkSpace();

/**
 * Helper function to get program accounts
 */
export async function getProgramAccounts(connection, programId) {
	try {
		const accounts = await connection.getProgramAccounts(programId);
		return accounts;
	} catch (error) {
		console.error('Error fetching program accounts:', error);
		return [];
	}
}

/**
 * Helper function to find PDA
 */
export async function findProgramAddress(seeds, programId) {
	const [publicKey, bump] = await PublicKey.findProgramAddress(seeds, programId);
	return { publicKey, bump };
}
