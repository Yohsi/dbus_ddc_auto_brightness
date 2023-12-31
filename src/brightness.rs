// This code was autogenerated with `dbus-codegen-rust `, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgKdeSolidPowerManagementActionsBrightnessControl {
    fn set_brightness(&self, arg0: i32) -> Result<(), dbus::Error>;
    fn set_brightness_silent(&self, arg0: i32) -> Result<(), dbus::Error>;
    fn brightness(&self) -> Result<i32, dbus::Error>;
    fn brightness_max(&self) -> Result<i32, dbus::Error>;
    fn brightness_steps(&self) -> Result<i32, dbus::Error>;
}

#[derive(Debug)]
pub struct OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessChanged {
    pub arg0: i32,
}

impl arg::AppendAll for OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
    }
}

impl arg::ReadAll for OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessChanged {
            arg0: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessChanged {
    const NAME: &'static str = "brightnessChanged";
    const INTERFACE: &'static str = "org.kde.Solid.PowerManagement.Actions.BrightnessControl";
}

#[derive(Debug)]
pub struct OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessMaxChanged {
    pub arg0: i32,
}

impl arg::AppendAll for OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessMaxChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
    }
}

impl arg::ReadAll for OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessMaxChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessMaxChanged {
            arg0: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessMaxChanged {
    const NAME: &'static str = "brightnessMaxChanged";
    const INTERFACE: &'static str = "org.kde.Solid.PowerManagement.Actions.BrightnessControl";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgKdeSolidPowerManagementActionsBrightnessControl for blocking::Proxy<'a, C> {

    fn set_brightness(&self, arg0: i32) -> Result<(), dbus::Error> {
        self.method_call("org.kde.Solid.PowerManagement.Actions.BrightnessControl", "setBrightness", (arg0, ))
    }

    fn set_brightness_silent(&self, arg0: i32) -> Result<(), dbus::Error> {
        self.method_call("org.kde.Solid.PowerManagement.Actions.BrightnessControl", "setBrightnessSilent", (arg0, ))
    }

    fn brightness(&self) -> Result<i32, dbus::Error> {
        self.method_call("org.kde.Solid.PowerManagement.Actions.BrightnessControl", "brightness", ())
            .and_then(|r: (i32, )| Ok(r.0, ))
    }

    fn brightness_max(&self) -> Result<i32, dbus::Error> {
        self.method_call("org.kde.Solid.PowerManagement.Actions.BrightnessControl", "brightnessMax", ())
            .and_then(|r: (i32, )| Ok(r.0, ))
    }

    fn brightness_steps(&self) -> Result<i32, dbus::Error> {
        self.method_call("org.kde.Solid.PowerManagement.Actions.BrightnessControl", "brightnessSteps", ())
            .and_then(|r: (i32, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>, )| Ok(r.0, ))
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, value, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}
