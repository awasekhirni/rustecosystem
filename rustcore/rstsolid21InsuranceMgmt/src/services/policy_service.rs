use crate::models::policy::{Policy, PolicyType};
use crate::traits::{Notifiable, Repository};

pub struct PolicyService<R, N>
where
    R: Repository<Policy, u32>,
    N: Notifiable,
{
    policy_repository: R,
    notifier: N,
}

impl<R, N> PolicyService<R, N>
where
    R: Repository<Policy, u32>,
    N: Notifiable,
{
    pub fn new(policy_repository: R, notifier: N) -> Self {
        PolicyService {
            policy_repository,
            notifier,
        }
    }

    pub fn create_policy(
        &mut self,
        customer_id: u32,
        policy_type: PolicyType,
        premium: f64,
    ) -> u32 {
        let policy = Policy::new(0, customer_id, policy_type, premium);
        let id = self.policy_repository.add(policy);

        self.notifier.send_notification(&format!(
            "New policy created with ID: {}. Premium: ${:.2}",
            id, premium
        ));

        id
    }

    pub fn get_policy(&self, id: u32) -> Option<Policy> {
        self.policy_repository.get_by_id(id)
    }

    pub fn get_all_policies(&self) -> Vec<Policy> {
        self.policy_repository.get_all()
    }
}
