#include "kf6/klocalizedstring.h"

namespace rust {
namespace kf6 {

void setApplicationDomain(const QByteArray &domain) {
  KLocalizedString::setApplicationDomain(domain);
}

} // namespace kf6
} // namespace rust
