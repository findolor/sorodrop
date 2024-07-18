import { MerkleTree } from "merkletreejs"
import { sha256 } from "js-sha256"

export interface Account {
  address: string
  amount: string
}

class Airdrop {
  private tree: MerkleTree

  constructor(accounts: Account[]) {
    const leaves = accounts.map((a) => sha256(`${a.address}${a.amount}`))
    this.tree = new MerkleTree(leaves, sha256, { sort: true })
  }

  getMerkleRoot() {
    return this.tree.getRoot().toString("hex")
  }

  getMerkleProof(account: Account) {
    return this.tree
      .getProof(sha256(`${account.address}${account.amount}`).toString())
      .map((p) => p.data.toString("hex"))
  }

  verify(proof: string[], account: Account) {
    return this.tree.verify(
      proof,
      sha256(`${account.address}${account.amount}`),
      this.tree.getRoot()
    )
  }
}

export default Airdrop
