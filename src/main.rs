use crate::ParcelStatus::{Delivered, Returned};

fn main() {
    let preparing = status_message(ParcelStatus::Preparing);
    println!("{}", preparing);
    let transit = status_message(ParcelStatus::InTransit(String::from("Aino")));
    println!("{}", transit);
    let delivered = status_message(ParcelStatus::Delivered(String::from("Mikko")));
    println!("{}", delivered);
    let returned = status_message(ParcelStatus::Returned(String::from("address not found")));
    println!("{}", returned);

    let note = Some("Leave the parcel at reception");
    print_delivery_note(note);
    let note2 = None;
    print_delivery_note(note2);

    announce_delivery(ParcelStatus::Delivered(String::from("Mikko")));
    announce_delivery(ParcelStatus::InTransit(String::from("Mikko")));

    create_receipt(ParcelStatus::Delivered(String::from("Mikko")));
    create_receipt(ParcelStatus::InTransit(String::from("Mikko")));
}

enum ParcelStatus {
    Preparing,
    InTransit(String),
    Delivered(String),
    Returned(String),
}

fn status_message(status: ParcelStatus) -> String {
    let parcel_status = match status {
        ParcelStatus::Preparing => String::from("Status: The parcel is being prepared."),
        ParcelStatus::InTransit(courier) => {
            format!("Status: The parcel is with courier {courier}.")
        }
        ParcelStatus::Delivered(recipient) => {
            format!("Status: The parcel was delivered to {recipient}.")
        }

        ParcelStatus::Returned(reason) => format!("Status: The parcel was returned: {reason}."),
    };
    parcel_status
}

fn print_delivery_note(note: Option<&str>) {
    if let Some(delivery_note) = note {
        println!("Delivery note: {}.", delivery_note);
    } else {
        println!("There is no delivery note.")
    }
}

fn announce_delivery(is_delivered: ParcelStatus) {
    if let ParcelStatus::Delivered(reciever) = is_delivered {
        println!("The parcel was delivered to {}.", reciever);
    } else {
        println!("The parcel has not been delivered yet.")
    }
}

fn create_receipt(delivered: ParcelStatus) {
    let ParcelStatus::Delivered(recipient) = delivered else {
        println!("Cannot create a receipt: the parcel was not delivered.");
        return;
    };

    println!("Receipt created for {}.", recipient)
}
