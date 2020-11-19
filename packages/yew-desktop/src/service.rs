/// Easily integrate desktop services
trait DesktopService {
    const NAME: &'static str;
    type ServiceRequest;
    type ServiceResponse;
    fn launch() -> Self;
    fn request_handler(&mut self, request: Self::ServiceResponse) -> Self::ServiceResponse;
}
