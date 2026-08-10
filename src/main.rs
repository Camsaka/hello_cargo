mod entry;
use entry::standards_actions;
use entry::admin_actions;
fn main() {
    standards_actions::create();
    standards_actions::get();
    standards_actions::update();
    standards_actions::delete();

    admin_actions::admin_connexion();
}
