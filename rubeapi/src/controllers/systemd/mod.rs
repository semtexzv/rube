use crate::api::systemd::unit::Unit;
use crate::controllers::Control;
use crate::Client;
use crate::Object;

pub struct SystemdController {

}

#[async_trait::async_trait]
impl Control<Unit> for SystemdController {
    async fn update(&mut self, client: &mut Client, obj: &Object<Unit>) {
        todo!()
    }

    async fn delete(&mut self, client: &mut Client, obj: &Object<Unit>) {
        todo!()
    }
}
