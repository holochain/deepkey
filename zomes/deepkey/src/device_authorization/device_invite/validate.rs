// @todo
// pub fn verify_signatures(&self) -> ExternResult<bool> {
//     let bytes = [self.root_acceptance.0.get_raw_32(), self.device_acceptance.0.get_raw_32()].concat();
//     Ok(
//         verify_signature_raw(self.root_acceptance.0.clone(), self.root_acceptance.1.clone(), bytes.to_vec())?
//         && verify_signature_raw(self.device_acceptance.0.clone(), self.device_acceptance.1.clone(), bytes.to_vec())?
//     )
// }