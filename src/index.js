const Web3 = require('web3');
const fs = require('fs');
const readline = require('readline');
const events = require('events');

let web3 = new Web3(process.env['ETH_NODE_RPC']);

const CONTROLLER_ADDRESS = '0x283Af0B28c62C092C9727F1Ee09c02CA627EB7F5';

const CONTROLLER_ABI = [{"inputs":[{"internalType":"contract BaseRegistrar","name":"_base","type":"address"},{"internalType":"contract PriceOracle","name":"_prices","type":"address"},{"internalType":"uint256","name":"_minCommitmentAge","type":"uint256"},{"internalType":"uint256","name":"_maxCommitmentAge","type":"uint256"}],"payable":false,"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"string","name":"name","type":"string"},{"indexed":true,"internalType":"bytes32","name":"label","type":"bytes32"},{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":false,"internalType":"uint256","name":"cost","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"expires","type":"uint256"}],"name":"NameRegistered","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"string","name":"name","type":"string"},{"indexed":true,"internalType":"bytes32","name":"label","type":"bytes32"},{"indexed":false,"internalType":"uint256","name":"cost","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"expires","type":"uint256"}],"name":"NameRenewed","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"oracle","type":"address"}],"name":"NewPriceOracle","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"constant":true,"inputs":[],"name":"MIN_REGISTRATION_DURATION","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"internalType":"string","name":"name","type":"string"}],"name":"available","outputs":[{"internalType":"bool","name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"internalType":"bytes32","name":"commitment","type":"bytes32"}],"name":"commit","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"name":"commitments","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"isOwner","outputs":[{"internalType":"bool","name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"address","name":"owner","type":"address"},{"internalType":"bytes32","name":"secret","type":"bytes32"}],"name":"makeCommitment","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"payable":false,"stateMutability":"pure","type":"function"},{"constant":true,"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"address","name":"owner","type":"address"},{"internalType":"bytes32","name":"secret","type":"bytes32"},{"internalType":"address","name":"resolver","type":"address"},{"internalType":"address","name":"addr","type":"address"}],"name":"makeCommitmentWithConfig","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"payable":false,"stateMutability":"pure","type":"function"},{"constant":true,"inputs":[],"name":"maxCommitmentAge","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"minCommitmentAge","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"duration","type":"uint256"},{"internalType":"bytes32","name":"secret","type":"bytes32"}],"name":"register","outputs":[],"payable":true,"stateMutability":"payable","type":"function"},{"constant":false,"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"duration","type":"uint256"},{"internalType":"bytes32","name":"secret","type":"bytes32"},{"internalType":"address","name":"resolver","type":"address"},{"internalType":"address","name":"addr","type":"address"}],"name":"registerWithConfig","outputs":[],"payable":true,"stateMutability":"payable","type":"function"},{"constant":false,"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"uint256","name":"duration","type":"uint256"}],"name":"renew","outputs":[],"payable":true,"stateMutability":"payable","type":"function"},{"constant":false,"inputs":[],"name":"renounceOwnership","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"uint256","name":"duration","type":"uint256"}],"name":"rentPrice","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"internalType":"uint256","name":"_minCommitmentAge","type":"uint256"},{"internalType":"uint256","name":"_maxCommitmentAge","type":"uint256"}],"name":"setCommitmentAges","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"internalType":"contract PriceOracle","name":"_prices","type":"address"}],"name":"setPriceOracle","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"internalType":"bytes4","name":"interfaceID","type":"bytes4"}],"name":"supportsInterface","outputs":[{"internalType":"bool","name":"","type":"bool"}],"payable":false,"stateMutability":"pure","type":"function"},{"constant":false,"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"internalType":"string","name":"name","type":"string"}],"name":"valid","outputs":[{"internalType":"bool","name":"","type":"bool"}],"payable":false,"stateMutability":"pure","type":"function"},{"constant":false,"inputs":[],"name":"withdraw","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"}];

const controller = () => new web3.eth.Contract(CONTROLLER_ABI, CONTROLLER_ADDRESS);

async function available(name) {
	return await controller().methods.available(name).call();
}

(async function() {
	if (process.argv.length > 2) {
		switch (process.argv[2]) {
			case 'single': {
				const is_available = await available(process.argv[3]);
				console.log(is_available ? 'yes' : 'no');
				break;
			}
			case 'multi': {
				const fstream = fs.createReadStream(process.argv[3]);
				const rl = readline.createInterface({input: fstream, crlfDelay: Infinity});
				rl.on('line', async (line) => {
					const name = line.trim().toLowerCase().replace(/[^a-z0-9]/g, '');
					if (name.length < 3) { return; }
					const avail = await available(name);
					if (avail === true) {
						console.log(name);
					}
				}).on('close', () => {
					console.log('done');
					process.exit(0);
				});
				await events.once(rl, 'close');
				break;
			}
			default: {
				console.error('Usage: [command] [single|multi] options');
				process.exit(1);
			}
		}
	}
	process.exit(0);
})();

