#include "cxx-kde-frameworks/kcrash.h"

namespace rust {
namespace kf6 {

void initializeKCrash() {
    KCrash::initialize();
}

} // namespace kf6
} // namespace rust
