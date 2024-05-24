import {
    Connection,
    Keypair,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';
import { BN } from 'bn.js';
import expect from 'expect';
import { Buffer } from "buffer";


function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    )
};


function getRandomInt() {
    const max = 255;
    let list = [];
    for (let i=0; i<8; i++) {
        list.push(Math.floor(Math.random() * max));
    }
    return new BN(Buffer.from(list));
};

describe("Create a system account", async () => {

    const connection = new Connection(`http://localhost:8899`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');

  
    it("Create the account", async () => {

        for(let i=0; i<1000000000; i++) {
            const a = getRandomInt();
            const b = getRandomInt();
    
            const buffers = [a.toBuffer("le", 8), b.toBuffer("le", 8)];
            const program_data = Buffer.concat(buffers);
            
            const c = a.mod(b);
            const c_buf = c.toBuffer("le", 8);
    
            let ix = new TransactionInstruction({
                keys: [
                    {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                    {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
                ],
                programId: program.publicKey,
                data: program_data,
            });
    
            const signature = await sendAndConfirmTransaction(
                connection, 
                new Transaction().add(ix),
                [payer]
            );
    
            let transaction = await connection.getTransaction(signature, {commitment: "confirmed"});
    
            const res = Buffer.from(transaction?.meta?.returnData.data[0], 'base64');
    
            expect(res).toEqual(c_buf);  
        }      
    });
  });
  