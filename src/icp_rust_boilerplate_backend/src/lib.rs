#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define memory and id cell types
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Ticket struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Ticket {
    id: u64,
    event_id: u64,
    user_id: u64,
    purchase_date: u64,
    seat_number: String,
    price: u64,
}

// Event struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Event {
    id: u64,
    name: String,
    location: String,
    date: u64,
    ticket_price: u64,
    total_tickets: u64,
    tickets_sold: u64,
}

// User struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    email: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
enum LoyaltyTier {
    #[default]
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct PointsTransaction {
    timestamp: u64,
    points: i64, // Can be negative for redemptions
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct LoyaltyPoints {
    user_id: u64,
    points: u64,
    tier: LoyaltyTier,
    points_history: Vec<PointsTransaction>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EventSeating {
    event_id: u64,
    vip_seats: Vec<String>,
    premium_seats: Vec<String>,
    standard_seats: Vec<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct EarlyAccessPass {
    user_id: u64,
    valid_until: u64,
    priority_level: u8,
}

// Payloads for API
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterUserPayload {
    username: String,
    email: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateEventPayload {
    name: String,
    location: String,
    date: u64,
    ticket_price: u64,
    total_tickets: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct PurchaseTicketPayload {
    event_id: u64,
    user_id: u64,
    seat_number: String,
}

// Message enum
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    InsufficientTickets(String),
}

// Implementing Storable and BoundedStorable for Ticket
impl Storable for Ticket {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for Ticket {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable and BoundedStorable for Event
impl Storable for Event {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for Event {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable and BoundedStorable for User
impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable and BoundedStorable for LoyaltyPoints
impl Storable for LoyaltyPoints {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LoyaltyPoints {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable and BoundedStorable for EventSeating
impl Storable for EventSeating {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EventSeating {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management using thread_local
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    static EVENTS_STORAGE: RefCell<StableBTreeMap<u64, Event, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    static TICKETS_STORAGE: RefCell<StableBTreeMap<u64, Ticket, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

    static LOYALTY_STORAGE: RefCell<StableBTreeMap<u64, LoyaltyPoints, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
        )
    );

    static SEATING_STORAGE: RefCell<StableBTreeMap<u64, EventSeating, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
        )
    );
}

// Function to register a user
#[ic_cdk::update]
fn register_user(payload: RegisterUserPayload) -> Result<User, Message> {
    if payload.username.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let user_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let user = User {
        id: user_id,
        username: payload.username,
        email: payload.email,
    };

    USERS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(user_id, user.clone());
    });

    Ok(user)
}

// Function to create an event
#[ic_cdk::update]
fn create_event(payload: CreateEventPayload) -> Result<Event, Message> {
    if payload.name.is_empty()
        || payload.location.is_empty()
        || payload.date == 0
        || payload.ticket_price == 0
        || payload.total_tickets == 0
    {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let event_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let event = Event {
        id: event_id,
        name: payload.name,
        location: payload.location,
        date: payload.date,
        ticket_price: payload.ticket_price,
        total_tickets: payload.total_tickets,
        tickets_sold: 0,
    };

    EVENTS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(event_id, event.clone());
    });

    Ok(event)
}

// Function to purchase a ticket
#[ic_cdk::update]
fn purchase_ticket(payload: PurchaseTicketPayload) -> Result<Ticket, Message> {
    if payload.seat_number.is_empty() {
        return Err(Message::InvalidPayload(
            "Seat number is required".to_string(),
        ));
    }

    EVENTS_STORAGE.with(|events| {
        let mut events = events.borrow_mut();
        if let Some(event) = events.get(&payload.event_id) {
            let mut updated_event = event.clone();

            if updated_event.tickets_sold >= updated_event.total_tickets {
                return Err(Message::InsufficientTickets(
                    "No tickets available".to_string(),
                ));
            }

            let ticket_id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("Counter increment failed");

            let ticket = Ticket {
                id: ticket_id,
                event_id: payload.event_id,
                user_id: payload.user_id,
                purchase_date: time(),
                seat_number: payload.seat_number,
                price: updated_event.ticket_price,
            };

            updated_event.tickets_sold += 1;
            events.insert(payload.event_id, updated_event);

            TICKETS_STORAGE.with(|tickets| {
                tickets.borrow_mut().insert(ticket_id, ticket.clone());
            });

            Ok(ticket)
        } else {
            Err(Message::NotFound("Event not found".to_string()))
        }
    })
}

// Function to list all events
#[ic_cdk::query]
fn list_all_events() -> Vec<Event> {
    EVENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, event)| event.clone())
            .collect()
    })
}

// Function to list tickets for a user
#[ic_cdk::query]
fn list_tickets_for_user(user_id: u64) -> Vec<Ticket> {
    TICKETS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, ticket)| ticket.user_id == user_id)
            .map(|(_, ticket)| ticket.clone())
            .collect()
    })
}

// Function to award points for ticket purchase
#[ic_cdk::update]
fn award_loyalty_points(user_id: u64, purchase_amount: u64) -> Result<LoyaltyPoints, Message> {
    let points_earned = calculate_points(purchase_amount);

    LOYALTY_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut loyalty = storage.get(&user_id).unwrap_or_default();

        loyalty.points += points_earned;
        loyalty.points_history.push(PointsTransaction {
            timestamp: time(),
            points: points_earned as i64,
            description: format!("Points earned from purchase: {}", purchase_amount),
        });

        // Update tier based on total points
        loyalty.tier = match loyalty.points {
            points if points >= 10000 => LoyaltyTier::Platinum,
            points if points >= 5000 => LoyaltyTier::Gold,
            points if points >= 2000 => LoyaltyTier::Silver,
            _ => LoyaltyTier::Bronze,
        };

        storage.insert(user_id, loyalty.clone());
        Ok(loyalty)
    })
}

// Function to redeem points for rewards
#[ic_cdk::update]
fn redeem_points(user_id: u64, points_to_redeem: u64) -> Result<String, Message> {
    LOYALTY_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut loyalty) = storage.get(&user_id) {
            if loyalty.points >= points_to_redeem {
                loyalty.points -= points_to_redeem;
                loyalty.points_history.push(PointsTransaction {
                    timestamp: time(),
                    points: -(points_to_redeem as i64),
                    description: "Points redemption".to_string(),
                });

                storage.insert(user_id, loyalty);
                Ok("Points successfully redeemed!".to_string())
            } else {
                Err(Message::Error("Insufficient points".to_string()))
            }
        } else {
            Err(Message::NotFound(
                "User loyalty account not found".to_string(),
            ))
        }
    })
}

// Modified ticket purchase function to include dynamic pricing
#[ic_cdk::update]
fn purchase_ticket_with_dynamic_pricing(payload: PurchaseTicketPayload) -> Result<Ticket, Message> {
    EVENTS_STORAGE.with(|events| {
        let mut events = events.borrow_mut();
        if let Some(event) = events.get(&payload.event_id) {
            let mut updated_event = event.clone();

            // Calculate dynamic price based on demand
            let demand_multiplier =
                (updated_event.tickets_sold as f64 / updated_event.total_tickets as f64) + 0.5;
            let dynamic_price = (updated_event.ticket_price as f64 * demand_multiplier) as u64;

            // Apply loyalty discount if applicable
            let final_price = LOYALTY_STORAGE.with(|storage| {
                if let Some(loyalty) = storage.borrow().get(&payload.user_id) {
                    match loyalty.tier {
                        LoyaltyTier::Platinum => dynamic_price * 80 / 100, // 20% discount
                        LoyaltyTier::Gold => dynamic_price * 85 / 100,     // 15% discount
                        LoyaltyTier::Silver => dynamic_price * 90 / 100,   // 10% discount
                        LoyaltyTier::Bronze => dynamic_price * 95 / 100,   // 5% discount
                    }
                } else {
                    dynamic_price
                }
            });

            // Create ticket with dynamic price
            let ticket_id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("Counter increment failed");

            let ticket = Ticket {
                id: ticket_id,
                event_id: payload.event_id,
                user_id: payload.user_id,
                purchase_date: time(),
                seat_number: payload.seat_number,
                price: final_price,
            };

            updated_event.tickets_sold += 1;
            events.insert(payload.event_id, updated_event);

            // Award loyalty points for purchase
            let _ = award_loyalty_points(payload.user_id, final_price);

            TICKETS_STORAGE.with(|tickets| {
                tickets.borrow_mut().insert(ticket_id, ticket.clone());
            });

            Ok(ticket)
        } else {
            Err(Message::NotFound("Event not found".to_string()))
        }
    })
}

// Helper function to calculate points
fn calculate_points(purchase_amount: u64) -> u64 {
    // Base rate: 1 point per 10 units spent
    let base_points = purchase_amount / 10;

    // Bonus points for larger purchases
    let bonus_points = match purchase_amount {
        amount if amount >= 1000 => base_points / 2, // 50% bonus
        amount if amount >= 500 => base_points / 4,  // 25% bonus
        amount if amount >= 200 => base_points / 10, // 10% bonus
        _ => 0,
    };

    base_points + bonus_points
}

// Exporting the candid interface
ic_cdk::export_candid!();
