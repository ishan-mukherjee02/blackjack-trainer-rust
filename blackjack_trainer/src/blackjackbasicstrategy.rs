pub struct BlackjackBasicStrategy {
  hard: [[i32; 10]; 17],
  soft: [[i32; 10]; 9],
  pair: [[i32; 10]; 10],
}
impl BlackjackBasicStrategy {
  pub fn new() -> Self {
      let hard = [
          [1, 1, 1, 1, 1, 1, 1, 1, 1, 1], //5
          [1, 1, 1, 1, 1, 1, 1, 1, 1, 1], //6
          [1, 1, 1, 1, 1, 1, 1, 1, 1, 1], //7
          [1, 1, 1, 1, 1, 1, 1, 1, 1, 1], //8
          [1, 3, 3, 3, 3, 1, 1, 1, 1, 1], //9
          [3, 3, 3, 3, 3, 3, 3, 3, 1, 1], //10
          [3, 3, 3, 3, 3, 3, 3, 3, 3, 1], //11
          [1, 1, 2, 2, 2, 1, 1, 1, 1, 1], //12
          [2, 2, 2, 2, 2, 1, 1, 1, 1, 1], //13
          [2, 2, 2, 2, 2, 1, 1, 1, 1, 1], //14
          [2, 2, 2, 2, 2, 1, 1, 1, 1, 1], //15
          [2, 2, 2, 2, 2, 1, 1, 1, 1, 1], //16
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //17
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //18
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //19
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //20
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //21
      ];

      let soft = [
          [1, 1, 1, 3, 3, 1, 1, 1, 1, 1], //13
          [1, 1, 1, 3, 3, 1, 1, 1, 1, 1], //14
          [1, 1, 3, 3, 3, 1, 1, 1, 1, 1], //15
          [1, 1, 3, 3, 3, 1, 1, 1, 1, 1], //16
          [1, 3, 3, 3, 3, 1, 1, 1, 1, 1], //17
          [2, 4, 4, 4, 4, 2, 2, 1, 1, 1], //18
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //19
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //20
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //21
      ];

      let pair = [
          [5, 5, 5, 5, 5, 5, 1, 1, 1, 1], //2, 2
          [5, 5, 5, 5, 5, 5, 1, 1, 1, 1], //3, 3
          [1, 1, 1, 5, 5, 1, 1, 1, 1, 1], //4, 4
          [3, 3, 3, 3, 3, 3, 3, 3, 1, 1], //5, 5
          [5, 5, 5, 5, 5, 1, 1, 1, 1, 1], //6, 6
          [5, 5, 5, 5, 5, 5, 1, 1, 1, 1], //7, 7
          [5, 5, 5, 5, 5, 5, 5, 5, 5, 5], //8, 8
          [5, 5, 5, 5, 5, 2, 5, 5, 2, 2], //9, 9
          [2, 2, 2, 2, 2, 2, 2, 2, 2, 2], //10, 10
          [5, 5, 5, 5, 5, 5, 5, 5, 5, 5], // A, A
      ];

      Self { hard, soft, pair }
  }

  pub fn get_correct_play(&self, hand: &[i32], dealer_card: i32) -> i32 {
      if self.check_if_pair(hand) {
          self.check_pairs(hand, dealer_card)
      } else if self.check_if_soft(hand) {
          self.check_soft_hands(hand, dealer_card)
      } else {
          self.check_hard_hands(hand, dealer_card)
      }
  }

  fn get_sum(&self, hand: &[i32]) -> Option<i32> {
      let mut sum = self.sum_aces(hand);

      for &card in hand {
          if card != 1 {
              sum += card;
          }
      }

      if sum > 21 {
          None
      } else {
          Some(sum)
      }
  }

  fn sum_aces(&self, hand: &[i32]) -> i32 {
      let num_ones = hand.iter().filter(|&&card| card == 1).count() as i32;
      let num_elevens = if num_ones > 0 { 1 } else { 0 };

      num_elevens * 11 + (num_ones - num_elevens)
  }

  fn check_if_soft(&self, hand: &[i32]) -> bool {
      self.sum_aces(hand) > 1
  }

  fn check_if_pair(&self, hand: &[i32]) -> bool {
      hand[0] == hand[1]
  }

  fn check_hard_hands(&self, hand: &[i32], dealer_card: i32) -> i32 {
      match self.get_sum(hand) {
          Some(sum) if sum >= 5 && sum <= 21 => self.hard[(sum - 5) as usize][(dealer_card - 2) as usize],
          _ => -1,
      }
  }

  fn check_soft_hands(&self, hand: &[i32], dealer_card: i32) -> i32 {
      match self.get_sum(hand) {
          Some(sum) if sum >= 13 && sum <= 21 => self.soft[(sum - 13) as usize][(dealer_card - 2) as usize],
          _ => -1,
      }
  }

  fn check_pairs(&self, hand: &[i32], dealer_card: i32) -> i32 {
      if hand[0] == 1 {
          5
      } else {
          self.pair[(hand[0] - 2) as usize][(dealer_card - 2) as usize]
      }
  }
}
