
// TODO:
/**
 * @brief Search to see if a card doesn't have a remaining pair left. Should be important to play
 */
fn is_critical<C: Card>(card : C) -> bool {
    false
}

/**
 * @brief Search to see if this card is unplayable this game because cards of the same color below
 * it have all been discarded
 */
fn is_dead<C: Card>(card : C) -> bool {
    false
}

