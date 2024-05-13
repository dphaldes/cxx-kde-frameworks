#include "kf6/klocalizedcontext.h"

namespace rust {
namespace kf6 {

void initializeEngine(QQmlEngine &engine) {
  engine.rootContext()->setContextObject(new KLocalizedContext(&engine));
}

} // namespace kf6
} // namespace rust
