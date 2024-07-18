import * as fs from "fs"
import Airdrop, { type Account } from "./merkle"

// Read the CSV file
const csvFilePath = "accounts.csv"
const csvContent = fs.readFileSync(csvFilePath, "utf-8")

// Parse the CSV content
const parseCSV = (csv: string): Account[] => {
  const lines = csv.trim().split("\n")
  const headers = lines[0].split(",")
  return lines.slice(1).map((line) => {
    const values = line.split(",")
    let obj: Account = {
      address: "",
      amount: "",
    }
    headers.forEach((header, index) => {
      obj[header as keyof Account] = values[index]
    })
    return obj
  })
}

const accounts = parseCSV(csvContent)

// Create an instance of Airdrop
const airdrop = new Airdrop(accounts)

const totalAmount = accounts.reduce(
  (sum, account) => sum + parseInt(account.amount),
  0
)
console.log("Total Amount:", totalAmount)

// Log the Merkle Root
console.log("Merkle Root:", airdrop.getMerkleRoot())
console.log("Merkle Proof for account 1:", airdrop.getMerkleProof(accounts[0]))
console.log("Merkle Proof for account 2:", airdrop.getMerkleProof(accounts[1]))
