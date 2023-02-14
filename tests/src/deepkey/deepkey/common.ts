

export async function sampleJoiningProof(cell: CallableCell, partialJoiningProof = {}) {
    return {
        ...{
	  keyset_proof: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  membrane_proof: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialJoiningProof
    };
}

export async function createJoiningProof(cell: CallableCell, joiningProof = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "deepkey",
      fn_name: "create_joining_proof",
      payload: joiningProof || await sampleJoiningProof(cell),
    });
}

