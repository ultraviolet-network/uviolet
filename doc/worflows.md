Fulfilling invoice:

| Action                              | Prime         | BP               | Input          | Output      | Method         | Components |
|-------------------------------------|---------------|------------------|----------------|-------------|----------------|------------|
| 1. Draft operation                  | mine PoW      |                  | Invoice        | OpPrefab    | prefab_op      | Stockpile  |
| 2. Create witness for the operation | not needed    | blanks, mine PoW | OpPrefab       | PSBT        | construct_psbt | Possessor  |
|                                     |               |                  | OpPerfab, PSBT | PSBT        | color_psbt     | Stockpile  |
| 4. Add operation to the stockpile   |               | ops              | OpPrefab       |             | import_prefab  | Stockpile  |
| 5. Prepare consignment              | removing sigs |                  | OpPrefab       | Consignment | consign        | Stockpile  |
| 6. Send consignment to receiver     |               |                  | Consignment    | OpConfirm   | -              | -          |
| 7a. Sign operation                  |               | not needed       | OpPrefab       | Operation   |                | Signer     |
| 7b. Sign witness                    | not needed    |                  | PSBT           | Tx          |                | Signer     |
| 8. Send to miners                   |               |                  | Request        | Witness     | -              | -          |
| 9. Store witness                    |               |                  | Witness        |             | import_witness | Pile       |
| 10. Store & send op to receiver     |               | not needed       | Operation      |             | import_op      | Stock      |

Multiparty procedures (like multisigs of payjoins) happen in step 3 (for BP) and step 5 (Prime, only
multisigs).

Consignment: must have a header defining type of seal. Should include witness information, and
a dedicated section on terminals.

Everything is in the stockpile:

- `stock`: partially-replicable state machine (PRISM) data, including:
    - `stash`: ledger of the contract operations (calls to contract state transitioning methods);
    - `state`: most recent contract state;
    - `trace`: incremental updates to the contract states from contract calls;
    - `repo`: repository of contract declarations and interfaces;
- `pile`: proof-of-publication data, store of witnesses and their links to operations, consisting of:
    - `hoard`: client-side witnesses (must be backed up, required for consensus);
    - `cache`: cache of published witnesses;
    - `index`: mapping of witnesses to operations (recoverable, but hard).

For read access, the only thing which is needed is contract state.
Wallet (possessor) acts as a prism (or a filter), allowing to see only the part of the state owned by the user.
Thus, the only form of a wallet we need for the read access is the seal filter.

On the sending side, we need wallet only when we prepare a witness for the operation, which is needed only in BP.

On the receiving side, we do not need wallet at all.
