//! [`Archive`] implementations for network types.

use crate::{offset_of, project_struct, Archive, Archived, Deserialize, Fallible, Serialize};
use core::{cmp, mem::MaybeUninit};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs},
};

/// An archived [`Ipv4Addr`](std::net::Ipv4Addr).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ArchivedIpv4Addr {
    octets: [u8; 4],
}

impl ArchivedIpv4Addr {
    /// Returns an [`Ipv4Addr`](std::net::Ipv4Addr) with the same value.
    #[inline]
    pub const fn as_ipv4(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.octets[0],
            self.octets[1],
            self.octets[2],
            self.octets[3],
        )
    }

    /// Returns `true` if this is a broadcast address (255.255.255.255).
    ///
    /// See [`Ipv4Addr::is_broadcast()`](std::net::Ipv4Addr::is_broadcast()) for more details.
    #[inline]
    pub const fn is_broadcast(&self) -> bool {
        self.as_ipv4().is_broadcast()
    }

    /// Returns `true` if this address is in a range designated for documentation.
    ///
    /// See [`Ipv4Addr::is_documentation()`](std::net::Ipv4Addr::is_documentation()) for more details.
    #[inline]
    pub const fn is_documentation(&self) -> bool {
        self.as_ipv4().is_documentation()
    }

    /// Returns `true` if the address is link-local (169.254.0.0/16).
    ///
    /// See [`Ipv4Addr::is_link_local()`](std::net::Ipv4Addr::is_link_local()) for more details.
    #[inline]
    pub const fn is_link_local(&self) -> bool {
        self.as_ipv4().is_link_local()
    }

    /// Returns `true` if this is a loopback address (127.0.0.0/8).
    ///
    /// See [`Ipv4Addr::is_loopback()`](std::net::Ipv4Addr::is_loopback()) for more details.
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        self.as_ipv4().is_loopback()
    }

    /// Returns `true` if this is a multicast address (224.0.0.0/4).
    ///
    /// See [`Ipv4Addr::is_multicast()`](std::net::Ipv4Addr::is_multicast()) for more details.
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        self.as_ipv4().is_multicast()
    }

    /// Returns `true` if this is a private address.
    ///
    /// See [`Ipv4Addr::is_private()`](std::net::Ipv4Addr::is_private()) for more details.
    #[inline]
    pub const fn is_private(&self) -> bool {
        self.as_ipv4().is_private()
    }

    /// Returns `true` for the special 'unspecified' address (0.0.0.0).
    ///
    /// See [`Ipv4Addr::is_unspecified()`](std::net::Ipv4Addr::is_unspecified()) for more details.
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        self.as_ipv4().is_unspecified()
    }

    /// Returns the four eight-bit integers that make up this address.
    #[inline]
    pub const fn octets(&self) -> [u8; 4] {
        self.octets
    }

    /// Converts this address to an IPv4-compatible [`IPv6` address](std::net::Ipv6Addr).
    ///
    /// See [`Ipv4Addr::to_ipv6_compatible()`](std::net::Ipv4Addr::to_ipv6_compatible()) for more
    /// details.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub const fn to_ipv6_compatible(&self) -> Ipv6Addr {
        self.as_ipv4().to_ipv6_compatible()
    }

    /// Converts this address to an IPv4-mapped [`IPv6` address](std::net::Ipv6Addr).
    ///
    /// See [`Ipv4Addr::to_ipv6_mapped()`](std::net::Ipv4Addr::to_ipv6_mapped()) for more details.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub const fn to_ipv6_mapped(&self) -> Ipv6Addr {
        self.as_ipv4().to_ipv6_mapped()
    }
}

impl Archive for Ipv4Addr {
    type Archived = ArchivedIpv4Addr;
    type Resolver = ();

    #[inline]
    fn resolve(&self, _: usize, _: Self::Resolver, out: &mut MaybeUninit<Self::Archived>) {
        unsafe {
            out.as_mut_ptr().cast::<[u8; 4]>().write(self.octets());
        }
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Ipv4Addr {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<Ipv4Addr, D> for ArchivedIpv4Addr {
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<Ipv4Addr, D::Error> {
        Ok(self.as_ipv4())
    }
}

impl PartialEq<Ipv4Addr> for ArchivedIpv4Addr {
    #[inline]
    fn eq(&self, other: &Ipv4Addr) -> bool {
        self.as_ipv4().eq(other)
    }
}

impl PartialEq<ArchivedIpv4Addr> for Ipv4Addr {
    #[inline]
    fn eq(&self, other: &ArchivedIpv4Addr) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<Ipv4Addr> for ArchivedIpv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<cmp::Ordering> {
        self.as_ipv4().partial_cmp(other)
    }
}

impl PartialOrd<ArchivedIpv4Addr> for Ipv4Addr {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedIpv4Addr) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

/// An archived [`Ipv6Addr`](std::net::Ipv6Addr).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ArchivedIpv6Addr {
    segments: [u16; 8],
}

impl ArchivedIpv6Addr {
    /// Returns an [`Ipv6Addr`](std::net::Ipv6Addr) with the same value.
    #[inline]
    pub const fn as_ipv6(&self) -> Ipv6Addr {
        Ipv6Addr::new(
            u16::from_be(self.segments[0]),
            u16::from_be(self.segments[1]),
            u16::from_be(self.segments[2]),
            u16::from_be(self.segments[3]),
            u16::from_be(self.segments[4]),
            u16::from_be(self.segments[5]),
            u16::from_be(self.segments[6]),
            u16::from_be(self.segments[7]),
        )
    }

    /// Returns `true` if this is a loopback address (::1).
    ///
    /// See [`Ipv6Addr::is_loopback()`](std::net::Ipv6Addr::is_loopback()) for more details.
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        self.as_ipv6().is_loopback()
    }

    /// Returns `true` if this is a multicast address (ff00::/8).
    ///
    /// See [`Ipv6Addr::is_multicast()`](std::net::Ipv6Addr::is_multicast()) for more details.
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        self.as_ipv6().is_multicast()
    }

    /// Returns `true` for the special 'unspecified' address (::).
    ///
    /// See [`Ipv6Addr::is_unspecified()`](std::net::Ipv6Addr::is_unspecified()) for more details.
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        self.as_ipv6().is_unspecified()
    }

    /// Returns the sixteen eight-bit integers the IPv6 address consists of.
    #[inline]
    pub const fn octets(&self) -> [u8; 16] {
        self.as_ipv6().octets()
    }

    /// Returns the eight 16-bit segments that make up this address.
    #[inline]
    pub const fn segments(&self) -> [u16; 8] {
        self.segments
    }

    /// Converts this address to an [`IPv4` address](std::net::Ipv4Addr). Returns
    /// [`None`](std::option::Option::None) if this address is neither IPv4-compatible or
    /// IPv4-mapped.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub const fn to_ipv4(&self) -> Option<Ipv4Addr> {
        self.as_ipv6().to_ipv4()
    }
}

impl Archive for Ipv6Addr {
    type Archived = ArchivedIpv6Addr;
    type Resolver = ();

    #[inline]
    fn resolve(&self, _: usize, _: Self::Resolver, out: &mut MaybeUninit<Self::Archived>) {
        unsafe {
            out.as_mut_ptr().cast::<[u8; 16]>().write(self.octets());
        }
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Ipv6Addr {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<Ipv6Addr, D> for ArchivedIpv6Addr {
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<Ipv6Addr, D::Error> {
        Ok(self.as_ipv6())
    }
}

impl PartialEq<Ipv6Addr> for ArchivedIpv6Addr {
    #[inline]
    fn eq(&self, other: &Ipv6Addr) -> bool {
        self.as_ipv6().eq(other)
    }
}

impl PartialEq<ArchivedIpv6Addr> for Ipv6Addr {
    #[inline]
    fn eq(&self, other: &ArchivedIpv6Addr) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<Ipv6Addr> for ArchivedIpv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<cmp::Ordering> {
        self.as_ipv6().partial_cmp(other)
    }
}

impl PartialOrd<ArchivedIpv6Addr> for Ipv6Addr {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedIpv6Addr) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

/// An archived [`IpAddr`](std::net::IpAddr).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ArchivedIpAddr {
    /// An IPv4 address.
    V4(ArchivedIpv4Addr),
    /// An IPv6 address.
    V6(ArchivedIpv6Addr),
}

impl ArchivedIpAddr {
    /// Returns an [`IpAddr`](std::net::IpAddr) with the same value.
    #[inline]
    pub const fn as_ipaddr(&self) -> IpAddr {
        match self {
            ArchivedIpAddr::V4(ipv4) => IpAddr::V4(ipv4.as_ipv4()),
            ArchivedIpAddr::V6(ipv6) => IpAddr::V6(ipv6.as_ipv6()),
        }
    }

    /// Returns `true` if this address is an [`IPv4` address](std::net::IpAddr::V4), and `false`
    /// otherwise.
    #[inline]
    pub const fn is_ipv4(&self) -> bool {
        matches!(self, ArchivedIpAddr::V4(_))
    }

    /// Returns `true` if this address is an [`IPv6` address](std::net::IpAddr::V6), and `false`
    /// otherwise.
    #[inline]
    pub const fn is_ipv6(&self) -> bool {
        matches!(self, ArchivedIpAddr::V6(_))
    }

    /// Returns `true` if this is a loopback address.
    ///
    /// See [`IpAddr::is_loopback()`](std::net::IpAddr::is_loopback()) for more details.
    #[inline]
    pub const fn is_loopback(&self) -> bool {
        match self {
            ArchivedIpAddr::V4(ip) => ip.is_loopback(),
            ArchivedIpAddr::V6(ip) => ip.is_loopback(),
        }
    }

    /// Returns `true` if this is a multicast address.
    ///
    /// See [`IpAddr::is_multicast()`](std::net::IpAddr::is_multicast()) for more details.
    #[inline]
    pub const fn is_multicast(&self) -> bool {
        match self {
            ArchivedIpAddr::V4(ip) => ip.is_multicast(),
            ArchivedIpAddr::V6(ip) => ip.is_multicast(),
        }
    }

    /// Returns `true` for the special 'unspecified' address.
    ///
    /// See [`IpAddr::is_unspecified()`](std::net::IpAddr::is_unspecified()) for more details.
    #[inline]
    pub const fn is_unspecified(&self) -> bool {
        match self {
            ArchivedIpAddr::V4(ip) => ip.is_unspecified(),
            ArchivedIpAddr::V6(ip) => ip.is_unspecified(),
        }
    }
}

#[allow(dead_code)]
#[repr(u8)]
enum ArchivedIpAddrTag {
    V4,
    V6,
}

#[repr(C)]
struct ArchivedIpAddrVariantV4(ArchivedIpAddrTag, ArchivedIpv4Addr);

#[repr(C)]
struct ArchivedIpAddrVariantV6(ArchivedIpAddrTag, ArchivedIpv6Addr);

impl Archive for IpAddr {
    type Archived = ArchivedIpAddr;
    type Resolver = ();

    #[inline]
    fn resolve(&self, pos: usize, resolver: Self::Resolver, out: &mut MaybeUninit<Self::Archived>) {
        match self {
            IpAddr::V4(ipv4_addr) => unsafe {
                let out = &mut *out
                    .as_mut_ptr()
                    .cast::<MaybeUninit<ArchivedIpAddrVariantV4>>();
                project_struct!(out: ArchivedIpAddrVariantV4 => 0: ArchivedIpAddrTag)
                    .as_mut_ptr()
                    .write(ArchivedIpAddrTag::V4);
                #[allow(clippy::unit_arg)]
                ipv4_addr.resolve(
                    pos + offset_of!(ArchivedIpAddrVariantV4, 1),
                    resolver,
                    project_struct!(out: ArchivedIpAddrVariantV4 => 1),
                );
            },
            IpAddr::V6(ipv6_addr) => unsafe {
                let out = &mut *out
                    .as_mut_ptr()
                    .cast::<MaybeUninit<ArchivedIpAddrVariantV6>>();
                project_struct!(out: ArchivedIpAddrVariantV6 => 0: ArchivedIpAddrTag)
                    .as_mut_ptr()
                    .write(ArchivedIpAddrTag::V6);
                #[allow(clippy::unit_arg)]
                ipv6_addr.resolve(
                    pos + offset_of!(ArchivedIpAddrVariantV6, 1),
                    resolver,
                    project_struct!(out: ArchivedIpAddrVariantV6 => 1),
                );
            },
        }
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for IpAddr {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        match self {
            IpAddr::V4(ipv4_addr) => ipv4_addr.serialize(serializer),
            IpAddr::V6(ipv6_addr) => ipv6_addr.serialize(serializer),
        }
    }
}

impl<D: Fallible + ?Sized> Deserialize<IpAddr, D> for Archived<IpAddr> {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<IpAddr, D::Error> {
        match self {
            ArchivedIpAddr::V4(ipv4_addr) => Ok(IpAddr::V4(ipv4_addr.deserialize(deserializer)?)),
            ArchivedIpAddr::V6(ipv6_addr) => Ok(IpAddr::V6(ipv6_addr.deserialize(deserializer)?)),
        }
    }
}

impl PartialEq<IpAddr> for ArchivedIpAddr {
    #[inline]
    fn eq(&self, other: &IpAddr) -> bool {
        match self {
            ArchivedIpAddr::V4(self_ip) => {
                if let IpAddr::V4(other_ip) = other {
                    self_ip.eq(other_ip)
                } else {
                    false
                }
            }
            ArchivedIpAddr::V6(self_ip) => {
                if let IpAddr::V6(other_ip) = other {
                    self_ip.eq(other_ip)
                } else {
                    false
                }
            }
        }
    }
}

impl PartialEq<ArchivedIpAddr> for IpAddr {
    #[inline]
    fn eq(&self, other: &ArchivedIpAddr) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<IpAddr> for ArchivedIpAddr {
    #[inline]
    fn partial_cmp(&self, other: &IpAddr) -> Option<cmp::Ordering> {
        self.as_ipaddr().partial_cmp(other)
    }
}

impl PartialOrd<ArchivedIpAddr> for IpAddr {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedIpAddr) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

/// An archived [`SocketAddrV4`](std::net::SocketAddrV4).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedSocketAddrV4 {
    ip: ArchivedIpv4Addr,
    port: u16,
}

impl ArchivedSocketAddrV4 {
    /// Returns a [`SocketAddrV4`](std::net::SocketAddrV4) with the same value.
    #[inline]
    pub fn as_socket_addr_v4(&self) -> SocketAddrV4 {
        SocketAddrV4::new(self.ip.as_ipv4(), self.port)
    }

    /// Returns the IP address associated with this socket address.
    #[inline]
    pub fn ip(&self) -> &ArchivedIpv4Addr {
        &self.ip
    }

    /// Returns the port number associated with this socket address.
    #[inline]
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl ToSocketAddrs for ArchivedSocketAddrV4 {
    type Iter = <SocketAddrV4 as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.as_socket_addr_v4().to_socket_addrs()
    }
}

impl Archive for SocketAddrV4 {
    type Archived = ArchivedSocketAddrV4;
    type Resolver = ();

    #[inline]
    fn resolve(&self, pos: usize, _: Self::Resolver, out: &mut MaybeUninit<Self::Archived>) {
        unsafe {
            self.ip().resolve(
                pos + offset_of!(ArchivedSocketAddrV4, ip),
                (),
                project_struct!(out: Self::Archived => ip),
            );
            self.port().resolve(
                pos + offset_of!(ArchivedSocketAddrV4, port),
                (),
                project_struct!(out: Self::Archived => port),
            );
        }
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for SocketAddrV4 {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<SocketAddrV4, D> for ArchivedSocketAddrV4 {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<SocketAddrV4, D::Error> {
        let ip = self.ip.deserialize(deserializer)?;
        Ok(SocketAddrV4::new(ip, self.port))
    }
}

impl PartialEq<SocketAddrV4> for ArchivedSocketAddrV4 {
    #[inline]
    fn eq(&self, other: &SocketAddrV4) -> bool {
        self.as_socket_addr_v4().eq(other)
    }
}

impl PartialEq<ArchivedSocketAddrV4> for SocketAddrV4 {
    #[inline]
    fn eq(&self, other: &ArchivedSocketAddrV4) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<SocketAddrV4> for ArchivedSocketAddrV4 {
    #[inline]
    fn partial_cmp(&self, other: &SocketAddrV4) -> Option<cmp::Ordering> {
        self.as_socket_addr_v4().partial_cmp(other)
    }
}

impl PartialOrd<ArchivedSocketAddrV4> for SocketAddrV4 {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedSocketAddrV4) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

/// An archived [`SocketAddrV6`](std::net::SocketAddrV6).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedSocketAddrV6 {
    ip: ArchivedIpv6Addr,
    port: u16,
    flowinfo: u32,
    scope_id: u32,
}

impl ArchivedSocketAddrV6 {
    /// Returns a [`SocketAddrV6`](std::net::SocketAddrV6) with the same value.
    #[inline]
    pub fn as_socket_addr_v6(&self) -> SocketAddrV6 {
        SocketAddrV6::new(self.ip.as_ipv6(), self.port, self.flowinfo, self.scope_id)
    }

    /// Returns the flow information associated with this address.
    ///
    /// See [`SocketAddrV6::flowinfo()`](std::net::SocketAddrV6::flowinfo()) for more details.
    #[inline]
    pub fn flowinfo(&self) -> u32 {
        self.flowinfo
    }

    /// Returns the IP address associated with this socket address.
    #[inline]
    pub fn ip(&self) -> &ArchivedIpv6Addr {
        &self.ip
    }

    /// Returns the port number associated with this socket address.
    #[inline]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the scope ID associated with this address.
    ///
    /// See [`SocketAddrV6::scope_id()`](std::net::SocketAddrV6::scope_id()) for more details.
    #[inline]
    pub fn scope_id(&self) -> u32 {
        self.scope_id
    }
}

impl ToSocketAddrs for ArchivedSocketAddrV6 {
    type Iter = <SocketAddrV6 as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.as_socket_addr_v6().to_socket_addrs()
    }
}

impl Archive for SocketAddrV6 {
    type Archived = ArchivedSocketAddrV6;
    type Resolver = ();

    #[inline]
    fn resolve(&self, pos: usize, _: Self::Resolver, out: &mut MaybeUninit<Self::Archived>) {
        unsafe {
            self.ip().resolve(
                pos + offset_of!(ArchivedSocketAddrV6, ip),
                (),
                project_struct!(out: Self::Archived => ip),
            );
            self.port().resolve(
                pos + offset_of!(ArchivedSocketAddrV6, port),
                (),
                project_struct!(out: Self::Archived => port),
            );
            self.flowinfo().resolve(
                pos + offset_of!(ArchivedSocketAddrV6, flowinfo),
                (),
                project_struct!(out: Self::Archived => flowinfo),
            );
            self.scope_id().resolve(
                pos + offset_of!(ArchivedSocketAddrV6, scope_id),
                (),
                project_struct!(out: Self::Archived => scope_id),
            )
        }
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for SocketAddrV6 {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<SocketAddrV6, D> for ArchivedSocketAddrV6 {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<SocketAddrV6, D::Error> {
        let ip = self.ip.deserialize(deserializer)?;
        Ok(SocketAddrV6::new(
            ip,
            self.port,
            self.flowinfo,
            self.scope_id,
        ))
    }
}

impl PartialEq<SocketAddrV6> for ArchivedSocketAddrV6 {
    #[inline]
    fn eq(&self, other: &SocketAddrV6) -> bool {
        self.as_socket_addr_v6().eq(other)
    }
}

impl PartialEq<ArchivedSocketAddrV6> for SocketAddrV6 {
    #[inline]
    fn eq(&self, other: &ArchivedSocketAddrV6) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<SocketAddrV6> for ArchivedSocketAddrV6 {
    #[inline]
    fn partial_cmp(&self, other: &SocketAddrV6) -> Option<cmp::Ordering> {
        self.as_socket_addr_v6().partial_cmp(other)
    }
}

impl PartialOrd<ArchivedSocketAddrV6> for SocketAddrV6 {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedSocketAddrV6) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

/// An archived [`SocketAddr`](std::net::SocketAddr).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ArchivedSocketAddr {
    /// An IPv4 socket address.
    V4(ArchivedSocketAddrV4),
    /// An IPv6 socket address.
    V6(ArchivedSocketAddrV6),
}

impl ArchivedSocketAddr {
    /// Returns a [`SocketAddr`](std::net::SocketAddr) with the same value.
    #[inline]
    pub fn as_socket_addr(&self) -> SocketAddr {
        match self {
            ArchivedSocketAddr::V4(addr) => SocketAddr::V4(addr.as_socket_addr_v4()),
            ArchivedSocketAddr::V6(addr) => SocketAddr::V6(addr.as_socket_addr_v6()),
        }
    }

    /// Returns the IP address associated with this socket address.
    #[inline]
    pub fn ip(&self) -> IpAddr {
        match self {
            ArchivedSocketAddr::V4(addr) => IpAddr::V4(addr.ip().as_ipv4()),
            ArchivedSocketAddr::V6(addr) => IpAddr::V6(addr.ip().as_ipv6()),
        }
    }

    /// Returns the port number associated with this socket address.
    #[inline]
    pub fn port(&self) -> u16 {
        match self {
            ArchivedSocketAddr::V4(addr) => addr.port(),
            ArchivedSocketAddr::V6(addr) => addr.port(),
        }
    }

    /// Returns `true` if the [IP address](std::net::IpAddr) in this `ArchivedSocketAddr` is an
    /// [`IPv4` address](std::net::IpAddr::V4), and `false` otherwise.
    #[inline]
    pub fn is_ipv4(&self) -> bool {
        matches!(self, ArchivedSocketAddr::V4(_))
    }

    /// Returns `true` if the [IP address](std::net::IpAddr) in this `ArchivedSocketAddr` is an
    /// [`IPv6` address](std::net::IpAddr::V6), and `false` otherwise.
    #[inline]
    pub fn is_ipv6(&self) -> bool {
        matches!(self, ArchivedSocketAddr::V6(_))
    }
}

impl ToSocketAddrs for ArchivedSocketAddr {
    type Iter = <SocketAddr as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.as_socket_addr().to_socket_addrs()
    }
}

#[allow(dead_code)]
#[repr(u8)]
enum ArchivedSocketAddrTag {
    V4,
    V6,
}

#[repr(C)]
struct ArchivedSocketAddrVariantV4(ArchivedSocketAddrTag, ArchivedSocketAddrV4);

#[repr(C)]
struct ArchivedSocketAddrVariantV6(ArchivedSocketAddrTag, ArchivedSocketAddrV6);

impl Archive for SocketAddr {
    type Archived = ArchivedSocketAddr;
    type Resolver = ();

    #[inline]
    fn resolve(&self, pos: usize, resolver: Self::Resolver, out: &mut MaybeUninit<Self::Archived>) {
        match self {
            SocketAddr::V4(socket_addr) => unsafe {
                let out = &mut *out
                    .as_mut_ptr()
                    .cast::<MaybeUninit<ArchivedSocketAddrVariantV4>>();
                project_struct!(out: ArchivedSocketAddrVariantV4 => 0: ArchivedSocketAddrTag)
                    .as_mut_ptr()
                    .write(ArchivedSocketAddrTag::V4);
                #[allow(clippy::unit_arg)]
                socket_addr.resolve(
                    pos + offset_of!(ArchivedSocketAddrVariantV4, 1),
                    resolver,
                    project_struct!(out: ArchivedSocketAddrVariantV4 => 1),
                );
            },
            SocketAddr::V6(socket_addr) => unsafe {
                let out = &mut *out
                    .as_mut_ptr()
                    .cast::<MaybeUninit<ArchivedSocketAddrVariantV6>>();
                project_struct!(out: ArchivedSocketAddrVariantV6 => 0: ArchivedSocketAddrTag)
                    .as_mut_ptr()
                    .write(ArchivedSocketAddrTag::V6);
                #[allow(clippy::unit_arg)]
                socket_addr.resolve(
                    pos + offset_of!(ArchivedSocketAddrVariantV6, 1),
                    resolver,
                    project_struct!(out: ArchivedSocketAddrVariantV6 => 1),
                );
            },
        }
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for SocketAddr {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        match self {
            SocketAddr::V4(socket_addr) => socket_addr.serialize(serializer),
            SocketAddr::V6(socket_addr) => socket_addr.serialize(serializer),
        }
    }
}

impl<D: Fallible + ?Sized> Deserialize<SocketAddr, D> for Archived<SocketAddr> {
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<SocketAddr, D::Error> {
        match self {
            ArchivedSocketAddr::V4(socket_addr) => {
                Ok(SocketAddr::V4(socket_addr.deserialize(deserializer)?))
            }
            ArchivedSocketAddr::V6(socket_addr) => {
                Ok(SocketAddr::V6(socket_addr.deserialize(deserializer)?))
            }
        }
    }
}

impl PartialEq<SocketAddr> for ArchivedSocketAddr {
    #[inline]
    fn eq(&self, other: &SocketAddr) -> bool {
        self.as_socket_addr().eq(other)
    }
}

impl PartialEq<ArchivedSocketAddr> for SocketAddr {
    #[inline]
    fn eq(&self, other: &ArchivedSocketAddr) -> bool {
        other.eq(self)
    }
}

impl PartialOrd<SocketAddr> for ArchivedSocketAddr {
    #[inline]
    fn partial_cmp(&self, other: &SocketAddr) -> Option<cmp::Ordering> {
        self.as_socket_addr().partial_cmp(other)
    }
}

impl PartialOrd<ArchivedSocketAddr> for SocketAddr {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedSocketAddr) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}
