import IPFS from 'ipfs-core'
import { createOrbitDB } from '@orbitdb/core'

;(async function initdb() {
  const ipfs = await IPFS.create()
  const orbitdb = await createOrbitDB({ ipfs })

  // Create / Open a database. Defaults to db type "events".
  const db = await orbitdb.open("hello")
  
  const address = db.address
  console.log(address)

  // Listen for updates from peers
  db.events.on("update", async entry => {
    console.log(entry)
    const all = await db.all()
    console.log(all)
  })

  // Add an entry
  const hash = await db.add("world")
  console.log(hash)

  // Query
  for await (const record of db.iterator()) {
    console.log(record)
  }
  
  await db.close()
  await orbitdb.stop()
})()


