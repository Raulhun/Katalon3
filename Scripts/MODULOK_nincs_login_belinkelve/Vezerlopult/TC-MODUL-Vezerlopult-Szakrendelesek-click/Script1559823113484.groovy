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

'Belepes a beallitasok menube'
WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_sidebaron'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/btn_Mentes_Bealitasokban'), 30)

'1. Szakrendelesek/Aktualis szakrendeles click'
WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/btn_Szakrendelesek'))

WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/btn_Aktualis_szakrendeles'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Szakrendelesek/btn_Szakerendelesek_Aktualis_szakrendeles'), 30)

WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/btn_Szakerendelesek_Aktualis_szakrendeles'))

WebUI.verifyElementVisible(findTestObject('Vezerlopult/Szakrendelesek/btn_Uj_szakrendeles_Osszes_szakrendeles'))

'2. Szakrendelesek/Osszes szakrendeles click'
WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/btn_Osszes_szakrendeles'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Szakrendelesek/btn_Uj_szakrendeles_Osszes_szakrendeles'), 30)

'2/a Szakrendelesek/Osszes szakrendeles/Új szakrendeles'
WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/btn_Uj_szakrendeles_Osszes_szakrendeles'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Szakrendelesek/btn_Szakerendelesek_Aktualis_szakrendeles'), 30)

WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/btn_Szakerendelesek_Aktualis_szakrendeles'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Szakrendelesek/btn_Uj_szakrendeles_Osszes_szakrendeles'), 30)

'2/b Szakrendelesek/Osszes szakrendeles/Szerkesztés'
WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/table_osszes_szakrend_tr3_td_2'))

WebUI.delay(1)

WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/btn_Szerkesztes_Osses_Szakrendeles'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Szakrendelesek/btn_Szakerendelesek_Aktualis_szakrendeles'), 30)

WebUI.click(findTestObject('Vezerlopult/Szakrendelesek/btn_Szakerendelesek_Aktualis_szakrendeles'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Szakrendelesek/btn_Uj_szakrendeles_Osszes_szakrendeles'), 30)

