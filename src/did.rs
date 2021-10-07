use identity::iota::Receipt;
use identity::iota::{ClientMap, TangleRef};
use identity::prelude::*;

pub async fn create_did() -> Result<(IotaDocument, KeyPair, Receipt)> {
    // Create a client instance to send messages to the Tangle.
    let client: ClientMap = ClientMap::new();

    // Generate a new Ed25519 public/private key pair.
    let keypair: KeyPair = KeyPair::new_ed25519()?;

    // Create a DID Document (an identity) from the generated key pair.
    let mut document: IotaDocument = IotaDocument::new(&keypair)?;

    // Sign the DID Document with the default authentication key.
    document.sign(keypair.private())?;

    println!("DID Document JSON > {:#}", document);

    // Publish the DID Document to the Tangle.
    let receipt: Receipt = client.publish_document(&document).await?;
    document.set_message_id(*receipt.message_id());

    println!("Publish Receipt > {:#?}", receipt);

    // Display the web explorer url that shows the published message.
    println!("DID Document Transaction > {}", receipt.message_url()?);

    Ok((document, keypair, receipt))
}
