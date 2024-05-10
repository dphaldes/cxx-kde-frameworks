#include "kf6/klocalizedstring.h"

namespace rust {
namespace kf6 {

void setApplicationDomain(const QByteArray &domain) {
  KLocalizedString::setApplicationDomain(domain);
}

QByteArray applicationDomain() { return KLocalizedString::applicationDomain(); }

} // namespace kf6
} // namespace rust
