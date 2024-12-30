#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static TEST_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
            MemoryManager::init(DefaultMemoryImpl::default())
        );
    }

    #[test]
    fn test_register_user() {
        let payload = RegisterUserPayload {
            username: "TestUser".to_string(),
            email: "testuser@example.com".to_string(),
        };

        let result = register_user(payload);
        assert!(result.is_ok());
        let user = result.unwrap();

        assert_eq!(user.username, "TestUser");
        assert_eq!(user.email, "testuser@example.com");
        assert!(user.id > 0);
    }

    #[test]
    fn test_create_event() {
        let payload = CreateEventPayload {
            name: "Test Event".to_string(),
            location: "Test Location".to_string(),
            date: 1700000000,
            ticket_price: 100,
            total_tickets: 50,
        };

        let result = create_event(payload);
        assert!(result.is_ok());
        let event = result.unwrap();

        assert_eq!(event.name, "Test Event");
        assert_eq!(event.location, "Test Location");
        assert_eq!(event.date, 1700000000);
        assert_eq!(event.ticket_price, 100);
        assert_eq!(event.total_tickets, 50);
    }

    #[test]
    fn test_purchase_ticket() {
        let event_payload = CreateEventPayload {
            name: "Ticket Event".to_string(),
            location: "Event Location".to_string(),
            date: 1700000001,
            ticket_price: 200,
            total_tickets: 10,
        };

        let event_result = create_event(event_payload);
        assert!(event_result.is_ok());
        let event = event_result.unwrap();

        let user_payload = RegisterUserPayload {
            username: "Buyer".to_string(),
            email: "buyer@example.com".to_string(),
        };

        let user_result = register_user(user_payload);
        assert!(user_result.is_ok());
        let user = user_result.unwrap();

        let ticket_payload = PurchaseTicketPayload {
            event_id: event.id,
            user_id: user.id,
            seat_number: "A1".to_string(),
        };

        let ticket_result = purchase_ticket(ticket_payload);
        assert!(ticket_result.is_ok());
        let ticket = ticket_result.unwrap();

        assert_eq!(ticket.event_id, event.id);
        assert_eq!(ticket.user_id, user.id);
        assert_eq!(ticket.seat_number, "A1");
        assert_eq!(ticket.price, event.ticket_price);
    }

    #[test]
    fn test_award_loyalty_points() {
        let user_payload = RegisterUserPayload {
            username: "LoyalUser".to_string(),
            email: "loyal@example.com".to_string(),
        };

        let user_result = register_user(user_payload);
        assert!(user_result.is_ok());
        let user = user_result.unwrap();

        let points_result = award_loyalty_points(user.id, 500);
        assert!(points_result.is_ok());
        let loyalty = points_result.unwrap();

        assert_eq!(loyalty.user_id, user.id);
        assert!(loyalty.points > 0);
        assert_eq!(loyalty.tier, LoyaltyTier::Bronze);
    }

    #[test]
    fn test_redeem_points() {
        let user_payload = RegisterUserPayload {
            username: "Redeemer".to_string(),
            email: "redeemer@example.com".to_string(),
        };

        let user_result = register_user(user_payload);
        assert!(user_result.is_ok());
        let user = user_result.unwrap();

        let _ = award_loyalty_points(user.id, 1000);

        let redeem_result = redeem_points(user.id, 100);
        assert!(redeem_result.is_ok());
        assert_eq!(redeem_result.unwrap(), "Points successfully redeemed!");
    }

    #[test]
    fn test_list_events_by_location() {
        let payload1 = CreateEventPayload {
            name: "Event 1".to_string(),
            location: "Location A".to_string(),
            date: 1700000002,
            ticket_price: 50,
            total_tickets: 20,
        };

        let payload2 = CreateEventPayload {
            name: "Event 2".to_string(),
            location: "Location A".to_string(),
            date: 1700000003,
            ticket_price: 60,
            total_tickets: 30,
        };

        create_event(payload1).unwrap();
        create_event(payload2).unwrap();

        let events = list_events_by_location("Location A".to_string());
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_get_event_details() {
        let payload = CreateEventPayload {
            name: "Detailed Event".to_string(),
            location: "Details Location".to_string(),
            date: 1700000004,
            ticket_price: 100,
            total_tickets: 40,
        };

        let event_result = create_event(payload);
        assert!(event_result.is_ok());
        let event = event_result.unwrap();

        let details_result = get_event_details(event.id);
        assert!(details_result.is_ok());
        let details = details_result.unwrap();

        assert_eq!(details.name, "Detailed Event");
        assert_eq!(details.location, "Details Location");
        assert_eq!(details.date, 1700000004);
    }

    #[test]
    fn test_list_all_events() {
        let payload1 = CreateEventPayload {
            name: "Event Alpha".to_string(),
            location: "Alpha Location".to_string(),
            date: 1700000005,
            ticket_price: 70,
            total_tickets: 25,
        };

        let payload2 = CreateEventPayload {
            name: "Event Beta".to_string(),
            location: "Beta Location".to_string(),
            date: 1700000006,
            ticket_price: 80,
            total_tickets: 35,
        };

        create_event(payload1).unwrap();
        create_event(payload2).unwrap();

        let events = list_all_events();
        assert!(events.len() >= 2);
    }

    #[test]
    fn test_list_tickets_for_user() {
        let user_payload = RegisterUserPayload {
            username: "TicketUser".to_string(),
            email: "ticketuser@example.com".to_string(),
        };

        let user_result = register_user(user_payload);
        assert!(user_result.is_ok());
        let user = user_result.unwrap();

        let event_payload = CreateEventPayload {
            name: "Ticket Event".to_string(),
            location: "Ticket Location".to_string(),
            date: 1700000007,
            ticket_price: 120,
            total_tickets: 15,
        };

        let event_result = create_event(event_payload);
        assert!(event_result.is_ok());
        let event = event_result.unwrap();

        let ticket_payload = PurchaseTicketPayload {
            event_id: event.id,
            user_id: user.id,
            seat_number: "B1".to_string(),
        };

        purchase_ticket(ticket_payload).unwrap();

        let tickets = list_tickets_for_user(user.id);
        assert_eq!(tickets.len(), 1);
    }
}
