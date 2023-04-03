

export async function sampleKeyGeneration(cell: CallableCell, partialKeyGeneration = {}) {
  return {
      ...{
  new_key: (await fakeAgentPubKey()),
  new_key_signing_of_author: (await fakeActionHash()),
      },
      ...partialKeyGeneration
  };
}

export async function createKeyGeneration(cell: CallableCell, keyGeneration = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "create_key_generation",
    payload: keyGeneration || await sampleKeyGeneration(cell),
  });
}



export async function sampleKeyRevocation(cell: CallableCell, partialKeyRevocation = {}) {
  return {
      ...{
  prior_key_registration: (await fakeActionHash()),
  revocation_authorization: [(await fakeActionHash())],
      },
      ...partialKeyRevocation
  };
}

export async function createKeyRevocation(cell: CallableCell, keyRevocation = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "create_key_revocation",
    payload: keyRevocation || await sampleKeyRevocation(cell),
  });
}



export async function sampleKeyRegistration(cell: CallableCell, partialKeyRegistration = {}) {
  return {
      ...{
  key_registration: { type: 'Update' },
      },
      ...partialKeyRegistration
  };
}

export async function createKeyRegistration(cell: CallableCell, keyRegistration = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "create_key_registration",
    payload: keyRegistration || await sampleKeyRegistration(cell),
  });
}



export async function sampleKeyAnchor(cell: CallableCell, partialKeyAnchor = {}) {
  return {
      ...{
  bytes: [10],
      },
      ...partialKeyAnchor
  };
}

export async function createKeyAnchor(cell: CallableCell, keyAnchor = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "deepkey",
    fn_name: "create_key_anchor",
    payload: keyAnchor || await sampleKeyAnchor(cell),
  });
}

