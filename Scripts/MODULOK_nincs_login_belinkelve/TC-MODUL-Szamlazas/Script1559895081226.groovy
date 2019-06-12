import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.By as By
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.annotation.Keyword as Keyword
import org.openqa.selenium.WebDriver as WebDriver
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.interactions.Actions as Actions

'felvesszük a drivert, meg az actions-t'
WebDriver driver = DriverFactory.getWebDriver()

WebDriver driver3 = DriverFactory.getWebDriver()

Actions action = new Actions(driver3)

'kivalasztunk egy pacienst'
WebElement Webtable = driver.findElement(By.id('patients-table'))

Webtable.findElement(By.xpath('//table[@id=\'patients-table\']/tbody/tr[2]/td[3]')).click()

WebUI.delay(2)

'click Szamlakiallitas - Kiallitunk egy szamlat'
WebUI.click(findTestObject('H_Szamlazas/btn_Szamlazas_sidebar'))

WebUI.click(findTestObject('H_Szamlazas/btn_Szamlakiallitas_Szamlazas'))

WebUI.waitForElementVisible(findTestObject('H_Szamlazas/page_Szamlakiallitas'), 30)

WebUI.delay(2)

'megnezzük van-e mar paciens a tablaban'
WebDriver driver2 = DriverFactory.getWebDriver()

WebElement Webtable2 = driver2.findElement(By.id('invoices-table'))

List<WebElement> TotalRowCount2 = Webtable2.findElements(By.xpath('//table[@id=\'invoices-table\']/tbody/tr'))

//KeywordUtil.markWarning('A találatok száma: ' + TotalRowCount2.size())
if (TotalRowCount2.size() >= 3) {
    //    KeywordUtil.markWarning('bent if')
    Webtable2.findElement(By.xpath('//table[@id=\'invoices-table\']/tbody/tr[2]/td[2]')).click( //    KeywordUtil.markWarning('bent else')
        )
} else {
    WebUI.click(findTestObject('H_Szamlazas/btn_Aktualis_paciens_Szamlakiallitas'))
}

WebUI.delay(2)

WebUI.waitForElementVisible(findTestObject('H_Szamlazas/btn_Tetel_hozaadasa_Szamlakiallitas'), 30)

WebUI.waitForElementClickable(findTestObject('H_Szamlazas/btn_Tetel_hozaadasa_Szamlakiallitas'), 30)

WebUI.click(findTestObject('H_Szamlazas/btn_Tetel_hozaadasa_Szamlakiallitas'))

WebUI.delay(2)

WebUI.waitForElementVisible(findTestObject('H_Szamlazas/modal_szamlatetel_kivalasztas'), 30)

WebUI.click(findTestObject('H_Szamlazas/td_szamlatetel_modalon'))

action.sendKeys(Keys.ESCAPE).build().perform()

WebUI.delay(2)

WebUI.waitForElementVisible(findTestObject('H_Szamlazas/btn_Szamla_kiallitasa_Szamlakiallitas'), 30)

WebUI.waitForElementClickable(findTestObject('H_Szamlazas/btn_Szamla_kiallitasa_Szamlakiallitas'), 30)

WebUI.click(findTestObject('H_Szamlazas/btn_Szamla_kiallitasa_Szamlakiallitas'))

WebUI.waitForPageLoad(60)

WebUI.switchToWindowIndex(0)

WebUI.closeWindowIndex(1)

WebUI.switchToWindowIndex(0)

'Paciens kialasztasa gomb proba - kivalasztunk egy masik pacienst'
WebUI.click(findTestObject('H_Szamlazas/btn_Szamlazas_sidebar'))

WebUI.click(findTestObject('H_Szamlazas/btn_Szamlakiallitas_Szamlazas'))

WebUI.waitForElementVisible(findTestObject('H_Szamlazas/page_Szamlakiallitas'), 30)

WebUI.waitForElementVisible(findTestObject('H_Szamlazas/btn_Paciens_kivalasztasa_Szamlakiallitas'), 30)

WebUI.click(findTestObject('H_Szamlazas/btn_Paciens_kivalasztasa_Szamlakiallitas'))

WebUI.waitForElementVisible(findTestObject('H_Szamlazas/td_paciens_kivalasztasa_modalon'), 30)

WebUI.waitForElementClickable(findTestObject('H_Szamlazas/td_paciens_kivalasztasa_modalon'), 30)

action.sendKeys(Keys.ESCAPE).build().perform()

WebUI.delay(2)

WebUI.click(findTestObject('H_Szamlazas/btn_Artetelek_Szamlazas'))

WebUI.verifyElementVisible(findTestObject('H_Szamlazas/page_Artetelek'))

WebUI.click(findTestObject('H_Szamlazas/btn_Kedvezmenyezett_csoportok_Szamlazas'))

WebUI.verifyElementVisible(findTestObject('H_Szamlazas/page_Kedvezmenyezett_csoportok'))

if (driver != null) {driver.quit()}
if (driver2 != null) {driver2.quit()}
if (driver3 != null) {driver3.quit()}