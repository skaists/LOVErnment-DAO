//! Smoke test: one public escrow-core API path exercised across the
//! tree boundary — Created → Funded via `Escrow::transition`, which runs
//! the kernel's §9.2 dual-balance check (denominated asset amount AND
//! the native ZANO fee buffer both observed at funding time).

use escrow_core::{Escrow, EscrowEvent, EscrowState, PublicKey, FEE_BUFFER};
use time::macros::datetime;

#[test]
fn kernel_escrow_funds_across_the_tree_boundary() {
    let created_at = datetime!(2026-07-07 12:00 UTC);
    let mut escrow = Escrow::new(
        "smoke-order-1",
        "smoke-msig-1",
        PublicKey([1u8; 32]),
        PublicKey([2u8; 32]),
        PublicKey([3u8; 32]),
        1_000_000,
        None,
        FEE_BUFFER,
        created_at,
    );
    assert_eq!(escrow.state, EscrowState::Created);

    let funded_at = datetime!(2026-07-07 13:00 UTC);
    let next = escrow
        .transition(EscrowEvent::BuyerFunded {
            asset_amount: 1_000_000,
            zano_amount: FEE_BUFFER,
            at: funded_at,
        })
        .expect("legal Created -> Funded transition within the 24h window");
    assert_eq!(next, EscrowState::Funded);
    assert_eq!(escrow.state, EscrowState::Funded);
    assert_eq!(escrow.funded_at, Some(funded_at));
}
